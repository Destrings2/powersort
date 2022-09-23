use powersort::{sequences::generate_random_sequence, sort::merge_sort};

fn main() {
    let sequence_sizes = [1000usize, 10000, 100000, 300000];
    let is_less = |a: &i32, b: &i32| a < b;

    for size in &sequence_sizes {
        let mut sequence = generate_random_sequence(*size);

        println!("Sequence size: {}", size);
        let mut clone = sequence.clone();
        println!("Default sort...");
        clone.sort();

        println!("(flawed) Power sort...");
        merge_sort(&mut sequence, is_less)
    }
}