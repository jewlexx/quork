pub trait ListVariants<const N: usize>
where
    Self: Sized,
{
    const VAIRANTS: [Self; N];
}
