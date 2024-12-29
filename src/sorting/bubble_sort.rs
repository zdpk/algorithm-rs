fn _bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

mod test {
    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![5, 3, 2, 4, 1];
        super::_bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
