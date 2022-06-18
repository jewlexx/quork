fn main() {
    let mut builder = cc::Build::new();

    if cfg!(target_os = "windows") {
        builder
            .files(vec!["src/windows/admin/admin.c"])
            .compile("windows");
    }
}
