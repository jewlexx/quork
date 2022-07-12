fn main() {
    let mut builder = cc::Build::new();

    if cfg!(windows) {
        builder
            .files(vec!["src/windows/admin/admin.c"])
            .compile("windows");
    }
}
