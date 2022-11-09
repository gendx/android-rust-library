#![cfg_attr(test, feature(never_type, test))]
#![cfg_attr(target_arch = "arm", feature(stdsimd))]
#![feature(let_chains, slice_as_chunks)]

mod cpu;
mod logger;
mod pmul;

#[cfg(test)]
extern crate test;

use cpu::{get_arch_name, print_cpu_features};
use logger::Logger;
use pmul::{pmul, pmul_cheat, pmul_nosimd};

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

    let (product, strategy) = pmul_nosimd(a, b);
    logger
        .d(format!(
            "pmul_nosimd({a:016x?}, {b:016x?}) = {product:032x?} [strategy = {strategy}]"
        ))
        .expect("Failed to log");

    let (product, strategy) = pmul_cheat(a, b);
    logger
        .d(format!(
            "pmul_cheat({a:016x?}, {b:016x?}) = {product:032x?} [strategy = {strategy}]"
        ))
        .expect("Failed to log");

    let (product, strategy) = pmul(a, b);
    logger
        .d(format!(
            "pmul({a:016x?}, {b:016x?}) = {product:032x?} [strategy = {strategy}]"
        ))
        .expect("Failed to log");

    your_arch
}
