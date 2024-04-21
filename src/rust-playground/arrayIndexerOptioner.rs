fn mulitplier(nums: Vec<usize>, index: usize) -> usize {
    return nums.get(index).unwrap_or(&index) * 5; 
}

fn main() {
    let vec = vec![1, 2, 3, 4];

    println!("value: {}", mulitplier(vec, 2));
}