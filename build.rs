fn main() {
    let mut builder = cc::Build::new();

    if cfg!(windows) {
        builder
            .file("src/windows/admin/admin.c")
            .compile("win_admin");

        cxx_build::bridge("src/windows/network.rs")
            .file("src/windows/network.cc")
            .flag_if_supported("-std=c++14")
            .compile("win_net");

        println!("cargo:rerun-if-changed=src/windows/admin/admin.c");
        println!("cargo:rerun-if-changed=src/windows/network.cc");
        println!("cargo:rerun-if-changed=include/windows/network.hpp");
    }
}
