// ryha-toolchain/ide/src/lib.rs

#[cfg(test)]
mod tests {
    #[test]
    fn test_ide_launch() {
        let status = std::process::Command::new("cargo")
            .arg("run")
            .arg("--package")
            .arg("ide")
            .arg("--")
            .arg("--help")
            .status()
            .unwrap();
        assert!(status.success());
    }
}
