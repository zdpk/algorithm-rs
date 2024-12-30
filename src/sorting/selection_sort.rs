fn _selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() - 1 {
        let mut min_idx = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            arr.swap(i, min_idx);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_selection_sort() {
        let mut arr = vec![5, 3, 2, 4, 1];
        super::_selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
