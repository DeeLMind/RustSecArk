fn main() {
    let nums = vec![1, 2, 3];
    let doubled: Vec<_> = nums.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled); // [2, 4, 6]
}
