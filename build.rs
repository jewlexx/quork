use cc::Build;

fn main() {
    if cfg!(windows) {
        println!("cargo:rustc-link-lib=ole32");

        Build::new()
            .file("include/win/network.c")
            .compile("networkcomid");
    }
}
