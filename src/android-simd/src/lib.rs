#![cfg_attr(test, feature(never_type))]

mod cpu;
mod logger;

use cpu::get_arch_name;
use logger::Logger;

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
pub mod test {
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

    your_arch
}
