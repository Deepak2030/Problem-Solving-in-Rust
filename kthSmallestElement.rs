fn quick_select(arr: &mut [i32], k: usize) -> i32 {
    if arr.len() == 1 {
        return arr[0];
    }

    let pivot_index = partition(arr);

    if k == pivot_index {
        return arr[pivot_index];
    } else if k < pivot_index {
        return quick_select(&mut arr[0..pivot_index], k);
    } else {
        return quick_select(&mut arr[pivot_index + 1..], k - pivot_index - 1);
    }
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);

    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

fn kth_smallest_element(arr: &[i32], k: usize) -> i32 {
    let mut cloned_arr = arr.to_vec();
    quick_select(&mut cloned_arr, k - 1)
}

fn main() {
    let arr = [3, 6, 2, 8, 4, 5];
    let k = 3;
    let kth_smallest = kth_smallest_element(&arr, k);
    println!("The {}th smallest element is: {}", k, kth_smallest);
}
