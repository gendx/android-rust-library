use crate::logger::Logger;
#[cfg(target_arch = "aarch64")]
use std::arch::asm;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

pub fn get_arch_name() -> &'static str {
    #[cfg(target_arch = "x86")]
    return "x86";

    #[cfg(target_arch = "x86_64")]
    return "x86_64";

    #[cfg(target_arch = "arm")]
    return "arm";

    #[cfg(target_arch = "aarch64")]
    return "aarch64";

    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64",
    )))]
    return "unknown";
}

fn cat_cpuinfo() -> io::Result<String> {
    let mut file = File::open("/proc/cpuinfo")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_cpuinfo_features() -> io::Result<HashSet<String>> {
    let file = File::open("/proc/cpuinfo")?;
    let reader = BufReader::new(file);

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    const EXPECTED_HEADER: &str = "flags";
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    const EXPECTED_HEADER: &str = "Features";

    let mut result = HashSet::new();
    for line in reader.lines() {
        let line = line?;
        let mut tokens = line.split_whitespace();
        if let Some(header) = tokens.next() && header == EXPECTED_HEADER {
            for token in tokens {
                if token != ":" {
                    result.insert(token.to_owned());
                }
            }
        }
    }
    Ok(result)
}

fn cat_auxv() -> io::Result<Vec<(usize, usize)>> {
    let mut file = File::open("/proc/self/auxv")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let (chunks, _remainder) = contents.as_chunks::<{ std::mem::size_of::<usize>() }>();
    let integers = chunks
        .iter()
        .map(|x| usize::from_ne_bytes(*x))
        .collect::<Vec<_>>();
    Ok(integers
        .as_chunks::<2>()
        .0
        .iter()
        .map(|&[key, value]| (key, value))
        .collect())
}

fn parse_auxv() -> io::Result<usize> {
    let mut file = File::open("/proc/self/auxv")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let (chunks, _remainder) = contents.as_chunks::<{ std::mem::size_of::<usize>() }>();
    let integers = chunks
        .iter()
        .map(|x| usize::from_ne_bytes(*x))
        .collect::<Vec<_>>();
    for &[key, value] in integers.as_chunks::<2>().0 {
        if key == 16 {
            return Ok(value);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::Other,
        "Invalid format for /proc/self/auxv",
    ))
}

// FIXME: Rust's libc doesn't have getauxval on "arm" for Android.
#[cfg(target_arch = "aarch64")]
fn get_auxval_hwcap() -> u64 {
    unsafe { libc::getauxval(16) }
}

// Note: This crashed on Android.
#[allow(dead_code)]
#[cfg(target_arch = "aarch64")]
fn parse_mrs() -> u64 {
    // ID_AA64ISAR0_EL1 - Instruction Set Attribute Register 0
    let aa64isar0: u64;
    unsafe {
        asm!(
            "mrs {}, ID_AA64ISAR0_EL1",
            out(reg) aa64isar0,
            options(pure, nomem, preserves_flags, nostack)
        );
    }
    aa64isar0
}

fn format_string_array<S: AsRef<str>>(values: &[S]) -> String {
    let mut result = String::new();
    result.push('[');
    for (i, x) in values.iter().enumerate() {
        if i != 0 {
            result.push_str(", ");
        }
        result.push_str(x.as_ref());
    }
    result.push(']');
    result
}

fn display_features<L: Logger>(
    logger: &L,
    enabled: &[&str],
    disabled: &[&str],
) -> Result<(), L::E> {
    logger.d(format!(
        "Detected {} enabled features:\n    {}",
        enabled.len(),
        format_string_array(enabled)
    ))?;
    logger.d(format!(
        "Detected {} disabled features:\n    {}",
        disabled.len(),
        format_string_array(disabled)
    ))?;
    Ok(())
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! print_x86_features {
    ( $logger:ident, $($feature:tt,)* ) => {
        let mut enabled = Vec::new();
        let mut disabled = Vec::new();
        $(
            if is_x86_feature_detected!($feature) {
                enabled.push($feature);
            } else {
                disabled.push($feature);
            }
        )*
        display_features($logger, &enabled, &disabled)?;
    }
}

#[cfg(target_arch = "arm")]
macro_rules! print_arm_features {
    ( $logger:ident, $($feature:tt,)* ) => {
        use std::arch::is_arm_feature_detected;
        let mut enabled = Vec::new();
        let mut disabled = Vec::new();
        $(
            if is_arm_feature_detected!($feature) {
                enabled.push($feature);
            } else {
                disabled.push($feature);
            }
        )*
        display_features($logger, &enabled, &disabled)?;
    }
}

#[cfg(target_arch = "aarch64")]
macro_rules! print_aarch64_features {
    ( $logger:ident, $($feature:tt,)* ) => {
        use std::arch::is_aarch64_feature_detected;
        let mut enabled = Vec::new();
        let mut disabled = Vec::new();
        $(
            if is_aarch64_feature_detected!($feature) {
                enabled.push($feature);
            } else {
                disabled.push($feature);
            }
        )*
        display_features($logger, &enabled, &disabled)?;
    }
}

pub fn print_cpu_features<L: Logger>(logger: &L) -> Result<(), L::E> {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    print_x86_features!(
        logger,
        "abm",
        "adx",
        "aes",
        "avx",
        "avx2",
        "avx512bf16",
        "avx512bitalg",
        "avx512bw",
        "avx512cd",
        "avx512dq",
        "avx512er",
        "avx512f",
        "avx512gfni",
        "avx512ifma",
        "avx512pf",
        "avx512vaes",
        "avx512vbmi",
        "avx512vbmi2",
        "avx512vl",
        "avx512vnni",
        "avx512vp2intersect",
        "avx512vpclmulqdq",
        "avx512vpopcntdq",
        "bmi1",
        "bmi2",
        "cmpxchg16b",
        "f16c",
        "fma",
        "fxsr",
        "lzcnt",
        "mmx",
        "pclmulqdq",
        "popcnt",
        "rdrand",
        "rdseed",
        "rtm",
        "sha",
        "sse",
        "sse2",
        "sse3",
        "sse4.1",
        "sse4.2",
        "sse4a",
        "ssse3",
        "tbm",
        "tsc",
        "xsave",
        "xsavec",
        "xsaveopt",
        "xsaves",
    );

    #[cfg(target_arch = "arm")]
    print_arm_features!(logger, "aes", "crc", "crypto", "i8mm", "neon", "pmull", "sha2",);

    #[cfg(target_arch = "aarch64")]
    print_aarch64_features!(
        logger,
        "aes",
        "asimd",
        "bf16",
        "bti",
        "crc",
        "dit",
        "dotprod",
        "dpb",
        "dpb2",
        "f32mm",
        "f64mm",
        "fcma",
        "fhm",
        "flagm",
        "fp",
        "fp16",
        "frintts",
        "i8mm",
        "jsconv",
        "lse",
        "lse2",
        "mte",
        "neon",
        "paca",
        "pacg",
        "pmull",
        "rand",
        "rcpc",
        "rcpc2",
        "rdm",
        "sb",
        "sha2",
        "sha3",
        "sm4",
        "ssbs",
        "sve",
        "sve2",
        "sve2-aes",
        "sve2-bitperm",
        "sve2-sha3",
        "sve2-sm4",
        "tme",
    );

    // Note: This crashed on Android.
    // #[cfg(target_arch = "aarch64")]
    // {
    //     let isar0 = parse_mrs();
    //     logger.d(format!(
    //         "Features found in Instruction Set Attribute Register 0: {:016x}",
    //         isar0
    //     ))?;
    // }

    #[cfg(target_arch = "aarch64")]
    {
        let hwcap = get_auxval_hwcap();
        logger.d(format!("HWCAP features found in getauxval: {:016x}", hwcap))?;
    }

    match parse_auxv() {
        Ok(hwcap) => logger.d(format!(
            "HWCAP features found in /proc/self/auxv ({} bits are set): {hwcap:0nibbles$x} / {hwcap:0bits$b}",
            hwcap.count_ones(),
            nibbles = 2 * std::mem::size_of::<usize>(),
            bits = 8 * std::mem::size_of::<usize>(),
        ))?,
        Err(e) => logger.d(format!("Failed to parse /proc/self/auxv: {:?}", e))?,
    }

    match parse_cpuinfo_features() {
        Ok(features) => {
            let mut sorted = features.iter().collect::<Vec<_>>();
            sorted.sort_unstable();
            logger.d(format!(
                "Found {} features in /proc/cpuinfo:\n    {}",
                sorted.len(),
                format_string_array(&sorted)
            ))?;
        }
        Err(e) => logger.d(format!("Failed to parse /proc/cpuinfo: {:?}", e))?,
    }

    match cat_auxv() {
        Ok(mut map) => {
            logger.d("Contents of /proc/self/auxv:")?;
            map.sort_by_key(|(key, _)| *key);
            for (key, value) in map {
                logger.d(format!(
                    "    {key:2} = {value:0nibbles$x} / {value:0bits$b}",
                    nibbles = 2 * std::mem::size_of::<usize>(),
                    bits = 8 * std::mem::size_of::<usize>(),
                ))?;
            }
        }
        Err(e) => logger.d(format!("Failed to read /proc/self/auxv: {:?}", e))?,
    }

    match cat_cpuinfo() {
        Ok(contents) => {
            logger.d(format!("Contents of /proc/cpuinfo:\n{}", contents))?;
        }
        Err(e) => logger.d(format!("Failed to read /proc/cpuinfo: {:?}", e))?,
    }

    Ok(())
}
