//! Quork macros module

pub use quork_proc::*;

#[macro_export]
/// A Rust implementation of the C `do while` loop
///
/// # Examples
///
/// ## Fizz Buzz
///
/// ```
/// # use quork::do_while;
/// let mut i = 1;
///
/// do_while!(
///     {
///         if i % 15 == 0 {
///             println!("fizzbuzz");
///         } else if i % 3 == 0 {
///             println!("fizz")
///         } else if i % 5 == 0 {
///             println!("Buzz");
///         } else {
///             println!("{i}");
///         }
///         i += 1;
///     },
///     i < 101
/// )
/// ```
macro_rules! do_while {
    ($code:block, $cond:expr) => {
        loop {
            $code

            if !$cond {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_do_while() {
        let mut i = 0;

        while false {
            i += 1;
        }

        assert_eq!(i, 0, "Regular while loop should not have run");

        i = 0;

        do_while!(
            {
                i += 1;
            },
            false
        );

        assert_eq!(i, 1, "Do while loop should have run once");
    }
}
