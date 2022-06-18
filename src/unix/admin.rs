/// Checks if the user is an administrator.
pub fn is_admin() -> bool {
    nix::unistd::geteuid().is_root()
}
