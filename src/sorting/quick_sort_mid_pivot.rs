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
    // 최초 pivot의 위치는 중앙값
    let pivot = arr[arr.len() / 2];

    let mut l = 0;
    let mut r = arr.len() - 1;

    loop {
        // 정렬이 안 된 배열에서 pivot보다 크거나 같은 값을 찾을 때까지 이동
        // pivot 보다 작은 값이 없는 경우도 있으므로 arr.len boundary check
        //
        // 만약 arr.len까지 도달한다면, pivot보다 큰 값이 없다는 의미
        // 이 경우, l == arr.len()이 되고 r == pivot이 됨
        // len 10이라면, l = 10, r = 5
        // pivot index는 5이므로, swap(5, 9) 후 9 반환하면 됨
        while arr[l] < pivot && l < arr.len() {
            l += 1;
        }

        // 정렬이 안 된 배열에서 pivot보다 작거나 같은 값을 찾음
        // pivot보다 큰 값이 없는 경우도 있으므로 boundary check
        //
        // 만약 0까지 도달한다면, pivot보다 작은 값이 없다는 의미
        // 이 경우 l == 0, r == pivot이 됨
        // len == 10인 경우 pivot index는 5이므로,
        // swap(5, 0) 후 0 반환하면 됨
        while arr[r] > pivot && r > 0 {
            r -= 1;
        }

        // check if pointers cross
        // if l greater than or equal to r
        // it means all elements on left are less than pivot
        // so return r as pivot index
        if l >= r {
            arr.swap(l, r);
            return r;
        }

        // l과 r의 값 교환
        arr.swap(l, r);
        l += 1;
        r -= 1;
    }
}

#[cfg(test)]
mod test {
    use crate::sorting::quick_sort_low_pivot::_quick_sort_hoare;

    #[test]
    fn test_quick_sort_hoare() {
        let mut arr = vec![5, 2, 8, 1, 9, 4, 7, 3, 6];
        _quick_sort_hoare(&mut arr);
        println!("{:?}", arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
