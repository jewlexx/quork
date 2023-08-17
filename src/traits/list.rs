pub trait ListVariants<const N: usize>
where
    Self: Sized,
{
    const VARIANTS: [Self; N];
}
