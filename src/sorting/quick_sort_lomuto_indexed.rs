fn _quick_sort_lomuto_indexed(arr: &mut [i32]) {
    if arr.len() > 1 {
        _quick_sort_lomuto_indexed_recursive(arr, 0, arr.len() - 1);
    }
}

fn _quick_sort_lomuto_indexed_recursive(arr: &mut [i32], low: usize, high: usize) {
    if low >= high {
        return;
    }
    let pivot_index = _partition(arr, low, high);

    if pivot_index > 0 {
        _quick_sort_lomuto_indexed_recursive(arr, low, pivot_index - 1);
    }
    _quick_sort_lomuto_indexed_recursive(arr, pivot_index + 1, high);
}

fn _partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    // 마지막 원소를 pivot으로 선택
    let pivot = arr[high];
    // i: pivot보다 작은 값들을 참조할 cursor
    let mut i = low;

    // j: 모든 원소를 순회할 cursor
    for j in low..high {
        // 현재 원소가 pivot보다 작은 경우 i, j에 위치한 원소 교환
        //
        // 1. arr[i] == arr[j]인 경우 skip
        // 2. arr[i] >  arr[j]인 경우, skip
        // 3. arr[i] <  arr[j]인 경우, 두 원소 위치 교환 및 i 증가
        // 결론적으로 3번 케이스 수만큼 i가 증가하며, i는 pivot보다 작은 값들의 개수를 나타냄
        // 또한 모두 pivot을 기준으로 왼쪽으로 정렬됨
        //
        // 즉, pivot보다 작은 값들은 왼쪽에 위치한다는 것을 보장
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    // 맨 뒤에 놓았던 pivot을 i 위치로 이동
    arr.swap(i, high);

    // i는 pivot의 index를 나타냄
    // pivot보다 작은 원소가 3개라면, i는 원소의 개수이자 pivot의 index이기 때문
    i
}

mod test {
    use crate::sorting::quick_sort_lomuto_indexed::_quick_sort_lomuto_indexed;

    #[test]
    fn test_quick_sort_lomuto() {
        let mut arr = vec![5, 2, 8, 1, 9, 4, 7, 3, 6];
        _quick_sort_lomuto_indexed(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
