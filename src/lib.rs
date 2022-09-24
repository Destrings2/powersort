pub mod sort;
pub mod sequences;
pub mod alternatives;
pub mod powersort_alternatives;
pub mod powersort;
pub mod powersort_final;

#[cfg(test)]
mod demonstrations {
    use rand::prelude::*;
    use rand_distr::Uniform;
    use rand::seq::SliceRandom;

    use crate::sequences::generate_random_sequence;

    use super::*;

    #[test]
    fn sorting() {

        // The power_sort is a generic function takes a slice of type [T] which implements the copy trait
        // and a function (T, T) -> bool which returns true if the first argument is less than the second.
        
        // This means any type T that has a function that can compare two elements can be sorted using 
        // this function.
        
        // For types that implement the Ord trait we can use the covenience function sort which uses the
        // comparison operator < by default.

        let mut i32_vec = generate_random_sequence(10000);
        let mut sorted_vec = i32_vec.clone();
        sorted_vec.sort();

        powersort::sort(&mut i32_vec);
        println!("First element: {}", i32_vec.first().unwrap());
        println!("Last element: {}", i32_vec.last().unwrap());
        assert_eq!(sorted_vec, i32_vec);

        // To sort decreasingly, we use a custom is_less function
        let mut i32_vec_2 = generate_random_sequence(10000);
        powersort::power_sort(&mut i32_vec_2, |&a, &b| a > b);
        println!("First element: {}", i32_vec_2.first().unwrap());
        println!("Last element: {}", i32_vec_2.last().unwrap());
    }

    #[test]
    fn custom_types() {        
        // Define a custom Chip type value field
        #[derive(Eq, PartialEq, Clone, Copy, std::fmt::Debug)]
        struct Chip {
            pub value: i32
        }

        impl Ord for Chip {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.value.cmp(&other.value)
            }
        }

        impl PartialOrd for Chip {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.value.cmp(&other.value))
            }
        }

        fn generate_random_chips(length: usize) -> Vec<Chip> {
            let mut vec: Vec<Chip> = Vec::with_capacity(length);
            let dist = Uniform::new_inclusive(1, 12);

            for _ in 0..length {
                let mut rng = thread_rng();
                let value = rng.sample(dist);
                
                vec.push(Chip {
                    value
                })
            }
            vec
        }

        let mut chips = generate_random_chips(25);
        println!("Chips: {:?}", chips);

        powersort::sort(&mut chips);
        println!("Chips: {:?}", chips);
    }
}