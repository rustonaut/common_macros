//! This library will contains common macros.
//!
//! Mainly following macros:
//!
//! - `hash_map!`,
//! - `hash_set!`,
//! - `b_tree_map!`,
//! - `b_tree_set!`,
//! - `expr_count!`
//! - `insert!` (generalization of map/set/vec insertion)
//!
//! I'm not sure if I will add common alt. assertion macros
//! (e.g. `assert_ok!`, `assert_err!`, `assert_iter_eq!`).
//!

#[macro_export]
macro_rules! const_expr_count {
    () => (0);
    ($e:expr) => (1);
    ($e:expr; $($other_e:expr);*) => ({
        1 $(+ $crate::const_expr_count!($other_e) )*
    });

    ($e:expr; $($other_e:expr);* ; ) => (
        $crate::const_expr_count! { $e; $($other_e);* }
    );
}


#[macro_export]
macro_rules! hash_map {
    ($($key:expr => $val:expr),* ,) => (
        $crate::hash_map!($($key => $val),*)
    );
    ($($key:expr => $val:expr),*) => ({
        let start_capacity = $crate::const_expr_count!($($key);*);
        let mut map = ::std::collections::HashMap::with_capacity(start_capacity);
        $( map.insert($key, $val); )*
        map
    });
}

#[macro_export]
macro_rules! hash_set {
    ($($item:expr),* ,) => (
        $crate::hash_set!($($item),*)
    );
    ($($item:expr),*) => ({
        let start_capacity = $crate::const_expr_count!($($item);*);
        let mut set = ::std::collections::HashSet::with_capacity(start_capacity);
        $( set.insert($item); )*
        set
    });
}

#[macro_export]
macro_rules! b_tree_map {
    ($($key:expr => $val:expr),* ,) => (
        $crate::b_tree_map!($($key => $val),*)
    );
    ($($key:expr => $val:expr),*) => ({
        let mut map = ::std::collections::BTreeMap::new();
        $( map.insert($key, $val); )*
        map
    });
}

#[macro_export]
macro_rules! b_tree_set {
    ($($item:expr),* ,) => (
        $crate::b_tree_set!($($item),*)
    );
    ($($item:expr),*) => ({
        let mut set = ::std::collections::BTreeSet::new();
        $( set.insert($item); )*
        set
    });
}

#[cfg(test)]
mod tests {

    mod const_expr_count {

        #[test]
        fn zero_expression() {
            assert_eq!(const_expr_count!{}, 0u8);
        }

        #[test]
        fn one_expression() {
            assert_eq!(const_expr_count!{1}, 1u8);
        }

        #[test]
        fn one_expression_with_semicolon() {
            assert_eq!(const_expr_count!{1;}, 1u8);
        }

        #[test]
        fn multiple_expressions() {
            assert_eq!(const_expr_count!{1; 1+2; (3+4, 5)}, 3u8);
        }

        #[test]
        fn multiple_expressions_with_trailing_semicolon() {
            assert_eq!(const_expr_count!{1; 1+2; (3+4, 5);}, 3u8);
        }
    }

    mod hash_map {
        use std::collections::HashMap;

        #[test]
        fn create_empty() {
            let map: HashMap<u8, u8> = hash_map!();
            assert_eq!(map.len(), 0);
        }

        #[test]
        fn create_non_empty() {
            let map = hash_map!{
                1u8 => 2u32
            };
            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.len(), 1);

            let map = hash_map!{
                1u8 => 2u32,
                4u8 => 12u32
            };
            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.get(&4), Some(&12));
            assert_eq!(map.len(), 2);
        }

        #[test]
        fn create_non_empty_with_tailing_comma() {
            let map = hash_map!{
                1u8 => 2u32,
            };
            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.len(), 1);
        }
    }

    mod hash_set {
        use std::collections::HashSet;

        #[test]
        fn create_empty() {
            let set: HashSet<u8> = hash_set!();
            assert_eq!(set.len(), 0);
        }

        #[test]
        fn create_non_empty() {
            let set = hash_set!{ 1u8 };
            assert!(set.contains(&1));
            assert_eq!(set.len(), 1);

            let set = hash_set!{ 1u8, 4u8 };
            assert!(set.contains(&1));
            assert!(set.contains(&4));
            assert_eq!(set.len(), 2);
        }

        #[test]
        fn create_non_empty_with_tailing_comma() {
            let set = hash_set!{ 1u8, };
            assert!(set.contains(&1));
            assert_eq!(set.len(), 1);
        }
    }

    mod b_tree_map {
        use std::collections::BTreeMap;

        #[test]
        fn create_empty() {
            let map: BTreeMap<u8, u8> = b_tree_map!();
            assert_eq!(map.len(), 0);
        }

        #[test]
        fn create_non_empty() {
            let map = b_tree_map!{
                1u8 => 2u32
            };
            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.len(), 1);

            let map = b_tree_map!{
                1u8 => 2u32,
                4u8 => 12u32
            };
            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.get(&4), Some(&12));
            assert_eq!(map.len(), 2);
        }

        #[test]
        fn create_non_empty_with_tailing_comma() {
            let map = b_tree_map!{
                1u8 => 2u32,
            };
            assert_eq!(map.get(&1), Some(&2));
            assert_eq!(map.len(), 1);
        }
    }

    mod b_tree_set {
        use std::collections::BTreeSet;

        #[test]
        fn create_empty() {
            let set: BTreeSet<u8> = b_tree_set!();
            assert_eq!(set.len(), 0);
        }

        #[test]
        fn create_non_empty() {
            let set = b_tree_set!{ 1u8 };
            assert!(set.contains(&1));
            assert_eq!(set.len(), 1);

            let set = b_tree_set!{ 1u8, 4u8 };
            assert!(set.contains(&1));
            assert!(set.contains(&4));
            assert_eq!(set.len(), 2);
        }

        #[test]
        fn create_non_empty_with_tailing_comma() {
            let set = b_tree_set!{ 1u8, };
            assert!(set.contains(&1));
            assert_eq!(set.len(), 1);
        }
    }
}