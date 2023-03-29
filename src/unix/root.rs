/// Checks if a user is root
pub fn is_root() -> bool {
    nix::unistd::Uid::effective().is_root()
}
