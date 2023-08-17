pub trait ListVariants<const N: usize> {
    const fn list_variants() -> [Self; N];
}
