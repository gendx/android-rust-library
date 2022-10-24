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
