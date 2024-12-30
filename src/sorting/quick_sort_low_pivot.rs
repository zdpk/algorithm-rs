fn _quick_sort_hoare(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = _partition(arr);
    _quick_sort_hoare(&mut arr[..pivot_index]);
    _quick_sort_hoare(&mut arr[pivot_index + 1..]);
}

// pivot을 기준으로 left는 작은 값, right는 큰 값으로 정렬 후
// pivot의 index를 반환
fn _partition(arr: &mut [i32]) -> usize {
    if arr.len() <= 1 {
        return 0;
    }

    let pivot = arr[0];

    let mut l = 1;
    let mut r = arr.len() - 1;

    loop {
        // move the left pointer until it finds an element that is greater than pivot
        while l < arr.len() && arr[l] <= pivot {
            l += 1;
        }

        // move the right pointer until it finds an element that is less than or equal to pivot
        while r > 0 && arr[r] > pivot {
            r -= 1;
        }

        // check if pointers cross
        // It means no elements left to move
        // so exchange the pivot and the element at l - 1
        //
        // Case 1. If every elements are less than pivot
        // - `l` will be `last`
        // - `r` will be `last - 1`
        // - so l > r, and `l - 1` or `r` is the index that `pivot` must be placed
        // because `l` moves next to the last element
        //
        // Case 2. If every elements are greater than pivot
        // - `l` will be 1
        // - `r` will be 0
        // - so l > r, and `l - 1` or `r` is the index that `pivot` must be placed
        //
        // Case 3. If there are 2 elements that are greater than pivot
        // - [4, 1, 6, 3, 7, 2, 5]
        // - pivot is 4
        // 1)
        // - `l` will be 1(val is 5)
        // - `r` will be 6(val is 1)
        // - swap them, [4, '5', 6, 3, 7, 2, '1']
        // 2)
        // - `l` will be 2(val is 6)
        // - `r` will be 5(val is 2)
        // - swap them, [4, 1, '2', 7, 3, '6', 5]
        //
        // 3)
        // - `l` will be 3(val is 7)
        // - `r` will be 4(val is 3)
        // - swap them, [4, 1, 2, '3', '7', 6, 5]
        //
        // 4)
        // - `l` will be 4(val is 7)
        // - `r` will be 3(val is 3)
        // - this time, l < r, so the loop will be terminated
        // - `l - 1` is the index that pivot must be placed
        //
        // eventually,
        // `l` stops at the element that is greater than pivot
        // `r` stops at the element that is less than or equal to pivot
        if l >= r {
            arr.swap(0, r);
            return r;
        }

        arr.swap(l, r);
    }
}

fn _get_pivot_index(arr: &[i32]) -> usize {
    let mid = arr.len() / 2; // 배열의 가운데 인덱스
    let last = arr.len() - 1; // 배열의 마지막 인덱스

    let a = arr[0]; // 첫 번째 요소
    let b = arr[mid]; // 중간 요소
    let c = arr[last]; // 마지막 요소

    if a <= b {
        // a <= b <= c -> 중간값은 b (중간 요소)
        if b <= c {
            mid
        }
        // a <= c < b -> 중간값은 c (마지막 요소)
        else if a <= c {
            last
        }
        // c < a <= b -> 중간값은 a (첫 번째 요소)
        else {
            0
        }
    } else {
        // b < a <= c -> 중간값은 a (첫 번째 요소)
        if a <= c {
            0
        }
        // b <= c < a -> 중간값은 c (마지막 요소)
        else if b <= c {
            last
        }
        // c < b < a -> 중간값은 b (중간 요소)
        else {
            mid
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![4, 5, 6, 3, 2, 1], vec![1,2,3,4,5,6])]
    #[case(vec![5, 2, 8, 1, 9, 4, 7, 3, 6], vec![1, 2, 3, 4, 5, 6, 7, 8, 9])]
    #[case(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5], vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9])]
    #[case(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])]
    #[case(vec![10, 9, 9, 9, 6, 5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5, 6, 9, 9, 9, 10])]
    #[case(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])]
    fn test_quick_sort_hoare(#[case] mut input: Vec<i32>, #[case] expected: Vec<i32>) {
        _quick_sort_hoare(&mut input);
        assert_eq!(input, expected);
    }
}
