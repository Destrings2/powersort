use sort::sort;

mod sort;
mod benchmark;

fn main() {
    let mut test = [71,71,56,16,72,53,76,93,19,97,71,36,95,18,69,65,99,100,39,92,98,60,5,22,61,81,84,48,12,81,61,26,51,91,71,62,50,68,87,62,88,75,12,53,23,82,51,64,8,69,64,82,54,75,86,40,7,84,63,87,14,60,65,9,32,30,28,90,38,57,91,16,11,43,83,23,76,42,46,98,93,27,94,83,77,36,29,43,87,34,69,83,3,73,88,35,1,31,75,77,];
    let mut test2 = test;
    println!("{:?}", test);
    sort(test.as_mut_slice());
    test2.sort();
    println!("{:?}", test2);
    println!("{:?}", test);
}