use cc::Build;

fn main() {
    if cfg!(windows) {
        println!("cargo:rustc-link-lib=ole32.lib");

        Build::new()
            .include("netlistmgr.h")
            .compile("netlistmgr.header");
    }
}
