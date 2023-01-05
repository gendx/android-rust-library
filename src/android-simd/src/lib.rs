#![cfg_attr(test, feature(never_type, test))]
#![cfg_attr(
    any(
        target_arch = "arm",
        all(test, feature = "relink", target_arch = "aarch64")
    ),
    feature(stdsimd)
)]
#![feature(let_chains, slice_as_chunks)]

#[cfg(all(feature = "relink", target_arch = "aarch64"))]
mod aes;
mod cpu;
#[cfg(all(feature = "relink", target_arch = "aarch64"))]
mod gf2n;
mod logger;
mod pmul;

#[cfg(test)]
extern crate test;

use cpu::{get_arch_name, print_cpu_features};
use logger::Logger;
use pmul::{pmul_strategy, pmul_strategy_cheat, pmul_strategy_nosimd};

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    use super::hello;
    use crate::logger::AndroidLogger;
    use jni::objects::JClass;
    use jni::sys::jstring;
    use jni::JNIEnv;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_myrustapplication_NativeLibrary_nativeRun(
        env: JNIEnv,
        _: JClass,
    ) -> jstring {
        println!("Hello from Rust's stdout. This message is sent to /dev/null by Android.");

        let logger = AndroidLogger::new(env, "MyRustSimdApplication")
            .expect("Couldn't create logger object");
        let your_arch = hello(&logger);

        let output = env
            .new_string(&your_arch)
            .expect("Couldn't create Java string!");
        output.into_raw()
    }
}

#[cfg(test)]
pub mod tests {
    use super::hello;
    use crate::logger::PrintlnLogger;

    #[test]
    fn test() {
        let logger = PrintlnLogger {};
        hello(&logger);
        // Hack to have the test's stdout be displayed.
        assert!(false);
    }
}

pub fn hello<L: Logger>(logger: &L) -> String {
    let your_arch = format!("Your CPU architecture is {}", get_arch_name());

    logger.d("Hello Rust world").expect("Failed to log");
    logger.d(&your_arch).expect("Failed to log");

    print_cpu_features(logger).expect("Failed to log");

    logger
        .d("Testing polynomial multiplication instructions")
        .expect("Failed to log");
    let a: u64 = 0x1234567890abcdef;
    let b: u64 = 0xfedcba0987654321;

    let (product, strategy) = pmul_strategy_nosimd(a, b);
    logger
        .d(format!(
            "pmul_nosimd({a:016x?}, {b:016x?}) = {product:032x?} [strategy = {strategy}]"
        ))
        .expect("Failed to log");

    let (product, strategy) = pmul_strategy_cheat(a, b);
    logger
        .d(format!(
            "pmul_cheat({a:016x?}, {b:016x?}) = {product:032x?} [strategy = {strategy}]"
        ))
        .expect("Failed to log");

    let (product, strategy) = pmul_strategy(a, b);
    logger
        .d(format!(
            "pmul({a:016x?}, {b:016x?}) = {product:032x?} [strategy = {strategy}]"
        ))
        .expect("Failed to log");

    #[cfg(all(feature = "relink", target_arch = "aarch64"))]
    {
        logger
            .d("Testing aesenc implementation")
            .expect("Failed to log");
        let src = [1; 16];
        let key = [2; 16];
        let mut dst = src;
        let strategy = aes::aesenc(&mut dst, &key);
        logger
            .d(format!(
                "aesenc({src:02x?}, {key:02x?}) = {dst:02x?} [strategy = {strategy}]"
            ))
            .expect("Failed to log");

        logger
            .d("Testing GF(2^n) implementation")
            .expect("Failed to log");

        let src = [1];
        let mut dst = src;
        gf2n::gf64_invert(&mut dst);
        logger
            .d(format!("invert({src:016x?}) = {dst:016x?}"))
            .expect("Failed to log");

        let src = [1, 2];
        let mut dst = src;
        gf2n::gf128_invert(&mut dst);
        logger
            .d(format!("invert({src:016x?}) = {dst:016x?}"))
            .expect("Failed to log");

        let src = [1, 2, 3, 4];
        let mut dst = src;
        gf2n::gf256_invert(&mut dst);
        logger
            .d(format!("invert({src:016x?}) = {dst:016x?}"))
            .expect("Failed to log");

        logger
            .d("Testing Shamir implementation")
            .expect("Failed to log");

        let src = [1; 32];
        let mut dst = [0; 640];
        gf2n::gf256_shamir_split_10(&src, &mut dst);
        logger
            .d(format!("gf256_shamir_split_10({src:02x?}) = {dst:02x?}"))
            .expect("Failed to log");
    }

    your_arch
}
