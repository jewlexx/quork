fn main() {
    let mut builder = cc::Build::new();

    if cfg!(windows) {
        builder
            .file("src/windows/admin/admin.c")
            .compile("win_admin");

        println!("cargo:rerun-if-changed=src/windows/admin/admin.c");
        println!("cargo:rerun-if-changed=src/windows/network.cc");
        println!("cargo:rerun-if-changed=include/windows/network.h");
    }
}
