fn _quick_sort_hoare_indexed(arr: &mut [i32]) {
    if arr.len() > 1 {
        _quick_sort_hoare_indexed_recursive(arr, 0, arr.len() - 1);
    }
}

fn _quick_sort_hoare_indexed_recursive(arr: &mut [i32], low: usize, high: usize) {
    if low > high {
        return;
    }

    let pivot_index = _partition(arr, low, high);

    if pivot_index > 0 {}
}

fn _partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    0
}
