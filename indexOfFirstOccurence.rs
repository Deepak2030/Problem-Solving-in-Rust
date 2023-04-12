fn find_first_occurrence(arr: &[i32], x: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;

        if arr[mid] < x {
            low = mid + 1;
        } else if arr[mid] > x {
            high = mid - 1;
        } else {
            // Found the element, now search for first occurrence
            let mut i = mid;
            while i > 0 && arr[i - 1] == x {
                i -= 1;
            }
            return Some(i);
        }
    }

    None
}

fn main() {
    let arr = [1, 2, 3, 4, 4, 5, 6];
    let x = 4;

    match find_first_occurrence(&arr, x) {
        Some(index) => println!("{} first occurs at index {}", x, index),
        None => println!("{} is not in the array", x),
    }
}
