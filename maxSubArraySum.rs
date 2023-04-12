fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_so_far = arr[0];
    let mut max_ending_here = arr[0];
    for i in 1..arr.len() {
        max_ending_here = std::cmp::max(arr[i], max_ending_here + arr[i]);
        max_so_far = std::cmp::max(max_so_far, max_ending_here);
    }
    max_so_far
}

fn main() {
    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&arr);
    println!("{}", max_sum);
}
