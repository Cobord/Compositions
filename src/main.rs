mod composition {
    use std::fmt::{Debug, Formatter, Result};
    pub struct Composition<T> {
        net: T,
        parts: Vec<T>,
        initial_value: T,
        fold_fun: fn(T, T) -> T,
        is_commutative: bool,
    }

    pub fn check_fold<T: Clone + std::cmp::PartialEq>(x: &Composition<T>) -> bool {
        x.net
            == x.parts.iter().fold(x.initial_value.clone(), |acc, b| {
                (x.fold_fun)(acc, (*b).clone())
            })
    }

    pub fn num_parts<T>(x: &Composition<T>) -> usize {
        x.parts.len()
    }

    pub fn singleton<T: Clone + PartialEq>(
        net: T,
        initial_value: T,
        fold_fun: fn(T, T) -> T,
        is_commutative: bool,
    ) -> Composition<T> {
        let z = vec![net.clone()];
        assert!(
            fold_fun(initial_value.clone(), net.clone()) == net,
            "Initial value must behave like identity"
        );
        Composition {
            net,
            parts: z,
            initial_value,
            fold_fun,
            is_commutative,
        }
    }

    pub fn construct<T: Clone>(
        parts: Vec<T>,
        initial_value: T,
        fold_fun: fn(T, T) -> T,
        is_commutative: bool,
    ) -> Composition<T> {
        let n = parts
            .iter()
            .fold(initial_value.clone(), |acc, b| fold_fun(acc, (*b).clone()));
        Composition {
            net: n,
            parts,
            initial_value,
            fold_fun,
            is_commutative,
        }
    }

    pub fn combine<T: std::cmp::PartialEq>(
        x: Composition<T>,
        y: Composition<T>,
        check_valid: bool,
    ) -> Composition<T> {
        assert!(
            !check_valid || x.initial_value == y.initial_value,
            "Both initial values must be the same" // but assumes that it is like identity
        );
        let my_initial_value = x.initial_value;
        assert!(
            !check_valid || x.fold_fun == y.fold_fun,
            "Both folding functions must be the same"
        );
        let my_fold_fun = x.fold_fun;
        let my_is_commutative = x.is_commutative && y.is_commutative;
        let mut xmod = x.parts;
        let mut ymod = y.parts;
        let mut z = Vec::with_capacity(xmod.len() + ymod.len());
        z.append(&mut xmod);
        z.append(&mut ymod);
        Composition {
            net: my_fold_fun(x.net, y.net),
            parts: z,
            initial_value: my_initial_value,
            fold_fun: my_fold_fun,
            is_commutative: my_is_commutative,
        }
    }

    pub fn decompose<T: std::cmp::PartialEq>(
        x: &mut Composition<T>,
        y: Composition<T>,
        idx: usize,
        check_valid: bool,
    ) {
        assert!(
            !check_valid || x.initial_value == y.initial_value,
            "Both initial values must be the same" // but assumes that it is like identity
        );
        assert!(
            !check_valid || x.fold_fun == y.fold_fun,
            "Both folding functions must be the same"
        );
        assert!(!check_valid || x.parts[idx] == y.net,
            "The net result of the composition to be inserted 
                    did not match with what the single part of the composition it was supposed to replace");
        x.parts.splice(idx..idx + 1, y.parts);
    }

    pub fn split<T: Clone + PartialEq>(
        x: &Composition<T>,
        predicate: fn(&T) -> bool,
    ) -> (Composition<T>, Composition<T>) {
        assert!(x.is_commutative,"The operation must be commutaive for the parts to be split 
                                    according to a proposition otherwise result changes upon recombining");
        assert!(x.initial_value.clone() == (x.fold_fun)(x.initial_value.clone(),x.initial_value.clone()),"
                                The initial value must be idempotent under the folding operation. It is defacto used in both factors of
                                the returned tuple");
        let (satisfiers, unsatisfiers): (Vec<T>, Vec<T>) =
            (*x).parts.iter().cloned().partition(predicate);
        let satisfying = construct(satisfiers, (*x).initial_value.clone(), (*x).fold_fun, true);
        let unsatisfying = construct(
            unsatisfiers,
            (*x).initial_value.clone(),
            (*x).fold_fun,
            true,
        );
        (satisfying, unsatisfying)
    }

    impl<T> Debug for Composition<T>
    where
        T: Debug + std::fmt::Display,
    {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(
                f,
                "{:?} of {} with {} parts",
                (*self).parts,
                (*self).net,
                num_parts(self)
            )
        }
    }

    pub struct Partition<T: Ord> {
        underlying_composition: Composition<T>,
    }

    pub fn to_partition<T: Ord>(mut me: Composition<T>) -> Partition<T> {
        assert!(
            me.is_commutative,
            "The operation must be commutative to go from composition to partition"
        );
        me.parts.sort();
        me.parts.reverse();
        Partition {
            underlying_composition: me,
        }
    }

    impl<T> Debug for Partition<T>
    where
        T: Debug + std::fmt::Display + Ord,
    {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "{:?}", self.underlying_composition)
        }
    }
}

mod test {
    #[allow(unused_imports)]
    use crate::composition;
    #[allow(unused_imports)]
    use composition::Composition;
    #[test]
    fn additive_four() {
        let add = |a, b| a + b;
        let x_1: Composition<u8> = composition::singleton(1, 0, add, true);
        assert!(composition::check_fold(&x_1));
        let x_1_copy = composition::singleton(1, 0, add, true);
        assert!(composition::check_fold(&x_1_copy));
        let x_2: Composition<u8> = composition::singleton(2, 0, add, true);
        assert!(composition::check_fold(&x_2));
        let x_12 = composition::combine(x_1, x_2, true);
        assert!(composition::check_fold(&x_12));
        let x_121 = composition::combine(x_12, x_1_copy, true);
        assert!(composition::check_fold(&x_121));
        assert_eq!(format!("{:?}", &x_121), "[1, 2, 1] of 4 with 3 parts");
    }
    #[test]
    fn additive_four_split_ones() {
        let add = |a, b| a + b;
        let x_1: Composition<u8> = composition::singleton(1, 0, add, true);
        let x_1_copy = composition::singleton(1, 0, add, true);
        let x_2: Composition<u8> = composition::singleton(2, 0, add, true);
        let x_12 = composition::combine(x_1, x_2, true);
        let x_121 = composition::combine(x_12, x_1_copy, true);
        let (x_1s, x_2s) = composition::split(&x_121, |z| (*z) == 1);
        assert_eq!(format!("{:?}", &x_121), "[1, 2, 1] of 4 with 3 parts");
        assert_eq!(format!("{:?}", &x_1s), "[1, 1] of 2 with 2 parts");
        assert_eq!(format!("{:?}", &x_2s), "[2] of 2 with 1 parts");
    }
    #[test]
    fn additive_four_decompose_two() {
        let add = |a, b| a + b;
        let x_1: Composition<u8> = composition::singleton(1, 0, add, true);
        let x_1_copy = composition::singleton(1, 0, add, true);
        let x_2: Composition<u8> = composition::singleton(2, 0, add, true);
        let x_12 = composition::combine(x_1, x_2, true);
        let mut x_121 = composition::combine(x_12, x_1_copy, true);
        let x_11 = composition::construct(vec![1, 1], 0, add, true);
        composition::decompose(&mut x_121, x_11, 1, true);
        assert!(composition::check_fold(&x_121));
        assert_eq!(format!("{:?}", &x_121), "[1, 1, 1, 1] of 4 with 4 parts");
    }
    #[test]
    fn partition_four() {
        let add = |a, b| a + b;
        let x_1: Composition<u8> = composition::singleton(1, 0, add, true);
        let x_1_copy = composition::singleton(1, 0, add, true);
        let x_2: Composition<u8> = composition::singleton(2, 0, add, true);
        let x_12 = composition::combine(x_1, x_2, true);
        let x_121 = composition::combine(x_12, x_1_copy, true);
        let x_211 = composition::to_partition(x_121);
        assert_eq!(format!("{:?}", &x_211), "[2, 1, 1] of 4 with 3 parts");
    }
}

fn main() {
    use composition::Composition;
    let add = |a, b| a + b;
    let x_1: Composition<u8> = composition::singleton(1, 0, add, true);
    assert!(composition::check_fold(&x_1));
    let x_1_copy = composition::singleton(1, 0, add, true);
    assert!(composition::check_fold(&x_1_copy));
    let x_2: Composition<u8> = composition::singleton(2, 0, add, true);
    assert!(composition::check_fold(&x_2));
    let x_12 = composition::combine(x_1, x_2, true);
    assert!(composition::check_fold(&x_12));
    let mut x_121 = composition::combine(x_12, x_1_copy, true);
    assert!(composition::check_fold(&x_121));
    println!("{:?}", x_121);
    let (x_1s, x_2s) = composition::split(&x_121, |z| (*z) == 1);
    println!(
        "{:?} splits to {:?} and {:?}",
        format!("{:?}", &x_121),
        format!("{:?}", &x_1s),
        format!("{:?}", &x_2s)
    );
    let x_11 = composition::construct(vec![1, 1], 0, add, true);
    assert!(composition::check_fold(&x_11));
    composition::decompose(&mut x_121, x_11, 1, true);
    assert!(composition::check_fold(&x_121));
    println!("{:?}", x_121);

    let x_11_fake: Composition<f32> =
        composition::construct(vec![1.0, 1.0], 0.0, |a, b| a + b, true);
    assert!(composition::check_fold(&x_11_fake));

    println!("Operation is now multiplication");
    let z_121 = composition::construct(vec![1, 2, 1], 1, |a, b| a * b, true);
    println!("{:?}", z_121);
    println!(
        "As a multiplicative partition it becomes {:?}",
        composition::to_partition(z_121)
    );

    println!("Operation is now string concatenation");
    let str_cat = |a: String, b: String| a + "\n" + &b;
    let greeting = composition::singleton("Hello".to_string(), "".to_string(), str_cat, false);
    let banter = composition::singleton("Banter".to_string(), "".to_string(), str_cat, false);
    let goodbye = composition::singleton("Bye".to_string(), "".to_string(), str_cat, false);
    let mut conversation = composition::combine(
        composition::combine(greeting, banter, false),
        goodbye,
        false,
    );
    println!("{:?}", conversation);
    let banter_refined = composition::construct(
        vec!["Banter 1".to_string(), "Banter 2".to_string()],
        "".to_string(),
        str_cat,
        false,
    );
    composition::decompose(&mut conversation, banter_refined, 1, false);
    println!("{:?}", conversation);
    if !composition::check_fold(&conversation) {
        println!("The lack of check valid means that it does not fold to give the exact same result as before");
    }
    /* errors because operation is not commutative so can't partition and rearrange steps freely
    let (_greeting_part,_rest) = composition::split(&conversation,|z| (*z)=="Hello"); */
}
