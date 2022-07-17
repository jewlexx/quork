fn main() {
    let mut builder = cc::Build::new();

    if cfg!(windows) {
        builder
            .file("src/windows/admin/admin.c")
            .compile("win_admin");

        cxx_build::bridge("src/windows/network.rs")
            .file("include/windows/check-net.hpp")
            .compile("win_net");
    }
}
