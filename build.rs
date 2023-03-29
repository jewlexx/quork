use cc::Build;

fn main() {
    if cfg!(windows) {
        Build::new()
            .include("netlistmgr.h")
            .compile("netlistmgr.header");
    }
}
