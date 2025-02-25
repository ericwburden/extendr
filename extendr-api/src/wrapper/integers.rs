use super::scalar::Rint;
use super::*;
use std::iter::FromIterator;

/// An obscure `NA`-aware wrapper for R's integer vectors.
/// Can be used to iterate over vectors obtained from R
/// or to create new vectors that can be returned back to R.
/// ```
/// use extendr_api::prelude::*;
/// test! {
///     let mut vec = (0..5).map(|i| i.into()).collect::<Integers>();
///     vec.iter_mut().for_each(|v| *v = *v + 10);
///     assert_eq!(vec.elt(0), 10);
///     let sum = vec.iter().sum::<Rint>();
///     assert_eq!(sum, 60);
/// }
/// ```  
#[derive(Debug, PartialEq, Clone)]
pub struct Integers {
    pub(crate) robj: Robj,
}

crate::wrapper::macros::gen_vector_wrapper_impl!(
    Integers, // Implements for
    Rint,     // Element type
    i32,      // Raw element type
    INTEGER,  // `R` functions prefix
    INTSXP,   // `SEXP`
    integer   // Singular type name used in docs
);

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn from_iterator() {
        test! {
            let vec : Integers = (0..3).map(|i| i.into()).collect();
            assert_eq!(vec, Integers::from_values([0, 1, 2]));
        }
    }

    #[test]
    fn iter_mut() {
        test! {
            let mut vec = Integers::from_values(0..3);
            vec.iter_mut().for_each(|v| *v = *v + 1);
            assert_eq!(vec, Integers::from_values(1..4));
        }
    }

    #[test]
    fn iter() {
        test! {
            let vec = Integers::from_values(0..3);
            assert_eq!(vec.iter().sum::<Rint>(), 3);
        }
    }

    #[test]
    fn from_values_short() {
        test! {
            // Short (<64k) vectors are allocated.
            let vec = Integers::from_values((0..3).map(|i| 2-i));
            assert_eq!(vec.is_altrep(), false);
            assert_eq!(r!(vec.clone()), r!([2, 1, 0]));
            assert_eq!(vec.elt(1), 1);
            let mut dest = [0; 2];
            vec.get_region(1, &mut dest);
            assert_eq!(dest, [1, 0]);
        }
    }

    #[test]
    fn from_values_long() {
        test! {
            // Long (>=64k) vectors a lazy ALTREP objects.
            let vec = Integers::from_values(0..1000000000);
            assert_eq!(vec.is_altrep(), true);
            assert_eq!(vec.elt(12345678), 12345678);
            let mut dest = [0; 2];
            vec.get_region(12345678, &mut dest);
            assert_eq!(dest, [12345678, 12345679]);
        }
    }

    #[test]
    fn new() {
        test! {
            let vec = Integers::new(10);
            assert_eq!(vec.is_integer(), true);
            assert_eq!(vec.len(), 10);
        }
    }
}
