use quork::traits::list::ListVariants;

#[derive(Debug, ListVariants, PartialEq, Eq)]
enum Variants {
    Var1,
    Var2,
    Var3,
}

#[test]
fn test_variants() {
    assert_eq!(
        Variants::VARIANTS,
        [Variants::Var1, Variants::Var2, Variants::Var3]
    )
}
