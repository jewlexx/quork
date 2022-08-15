pub fn is_admin() -> bool {
    nix::unistd::geteuid().is_root()
}
