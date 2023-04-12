fn median(arr: &[i32]) -> f32 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (arr[mid - 1] as f32 + arr[mid] as f32) / 2.0
    } else {
        arr[len / 2] as f32
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5, 6];

    println!("The median of {:?} is {}", arr1, median(&arr1));
    println!("The median of {:?} is {}", arr2, median(&arr2));
}
