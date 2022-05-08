fn main() {
    let mut builder = cc::Build::new();

    if cfg!(target_os = "windows") {
        builder
            .files(vec![
                "src/windows/admin/admin.c",
                "src/windows/network/network.c",
            ])
            .compile("windows");
    } else {
        builder
            .files(vec!["src/unix/admin/admin.c"])
            .compile("unix");
    }
}
