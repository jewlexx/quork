fn main() {
    let mut builder = cc::Build::new();

    if cfg!(windows) {
        builder
            .file("src/windows/admin/admin.c")
            .compile("win_admin");
    }
}
