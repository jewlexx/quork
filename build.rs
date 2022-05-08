fn main() {
    let mut builder = cc::Build::new();

    if cfg!(target_os = "windows") {
        builder
            .cpp(true)
            .files(vec![
                "src/windows/admin/admin.c",
                "src/windows/network/network.cpp",
            ])
            .compile("windows");
    } else {
        builder
            .files(vec!["src/unix/admin/admin.c"])
            .compile("unix");
    }
}
