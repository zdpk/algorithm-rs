fn _merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    _merge_sort(&mut arr[..mid]);
    _merge_sort(&mut arr[mid..]);
    _merge(arr, mid);
}

// fn _merge_sort_iter(arr: &mut [i32]) {
//     let len = arr.len();
//     let mut temp = arr.to_vec();
//     let mut width = 1;

//     // width를 1씩 늘려주면서 merge
//     while width < len {
//         // width가 n일 때, len까지 돌면서 모든 subarray를 merge
//         let mut i = 0;
//         while i < len {
//             // mid가 len보다 커지지 않도록 제한
//             // width 2일 때, mid는 2, 4, 6, 8, ... 이런 식으로 증가
//             // len = 5일 때
//             // i = 0, mid = 2, end = 4
//             // i = 1, mid = 3, end = 5
//             // i = 2, mid = 4, end = 5
//             let mid = std::cmp::min(i + width, len);
//             let end = std::cmp::min();
//         }
//     }
// }

fn _merge(arr: &mut [i32], mid: usize) {
    // 1. divide into 2 subarrays until the size of the subarray is 1
    let left = &arr[..mid].to_vec();
    let right = &arr[mid..].to_vec();

    let mut l = 0;
    let mut r = 0;
    let mut a = 0;

    // merge sort 시, 재귀 호출을 통해 처음 실행되는 _merge는
    // left, right가 각각 1개의 원소를 가지고 있음
    // 즉, 이후 호출에서도 left, right는 항상 정렬된 상태로 들어옴
    // 따라서 left, right를 arr로 합쳤을 때도 정렬되도록 만드는 단계
    while l < left.len() && r < right.len() {
        if left[l] <= right[r] {
            arr[a] = left[l];
            l += 1;
        } else {
            arr[a] = right[r];
            r += 1;
        }
        a += 1;
    }

    // arr: [1, 3, 5, 6, 2, 4, 7, 8]
    // left: [2, 4, 7, 8]
    // right: [1, 3, 5, 6]
    // 위와 같은 상태에서 _merge 호출 시,
    // l = 2, r = 4, a = 6
    // arr: [1, 2, 3, 4, 7, 8, 5, 6]이 됨
    // 이 때, right의 남은 원소를 arr에 추가해줘야 함
    while l < left.len() {
        arr[a] = left[l];
        l += 1;
        a += 1;
    }

    while r < right.len() {
        arr[a] = right[r];
        r += 1;
        a += 1;
    }
}

#[cfg(test)]
mod test {
    use crate::sorting::merge_sort::_merge_sort;

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![5, 2, 8, 1, 9, 4, 7, 3, 6];
        _merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
