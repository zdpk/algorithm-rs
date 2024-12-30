fn _insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        // save the key, it can be overwritten by previous elements
        let mut key = arr[i];
        let mut j: isize = (i as isize) - 1;
        // until j is not negative and the element is greater than the key
        // so find the right position for the key
        while j >= 0 && arr[j as usize] > key {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![5, 3, 2, 4, 1];
        super::_insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
