use quork_proc::ListVariants;

#[derive(ListVariants)]
enum Variants {
    Var1,
    Var2,
    Var3,
}

#[test]
fn test_variants() {}
