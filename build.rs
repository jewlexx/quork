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
        let libnm = pkg_config::probe_library("libnm").unwrap();

        builder
            .files(vec!["src/unix/admin/admin.c", "src/unix/network/network.c"])
            .includes(libnm.include_paths)
            .compile("unix");
    }
}
