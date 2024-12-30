fn _heap_sort(arr: &mut [i32]) {
    let heap_size = arr.len();
    // 1. Build Max Heap
    for i in (0..arr.len() / 2).rev() {
        _heapify(arr, heap_size, i);
    }

    // 2. Sort
    for i in (0..arr.len()).rev() {
        arr.swap(i, 0);
        _heapify(arr, heap_size, 0);
    }
}

fn _heapify(arr: &mut [i32], heap_size: usize, root_index: usize) {
    let mut largest_index = root_index;
    let left_index = 2 * root_index + 1;
    let right_index = 2 * root_index + 2;

    if left_index < heap_size && arr[left_index] > arr[root_index] {
        largest_index = left_index;
    }

    if right_index < heap_size && arr[right_index] > arr[root_index] {
        largest_index = right_index;
    }

    if largest_index != root_index {
        arr.swap(root_index, largest_index);
        _heapify(arr, heap_size, largest_index);
    }
}
// fn _heap_sort(arr: &mut [i32]) {
//     let heap_size = arr.len();

//     // 배열을 최대 힙으로 변환 (Build Max Heap)
//     for root_idx in (0..heap_size / 2).rev() {
//         _heapify(arr, heap_size, root_idx);
//     }

//     print_tree(arr);

//     println!("initial heap");

//     // 힙에서 원소를 하나씩 추출하여 정렬 (Sort)
//     for i in (1..heap_size).rev() {
//         arr.swap(0, i); // 루트 노드 (가장 큰 값)와 마지막 노드 교환
//         _heapify(arr, i, 0); // 힙 크기를 줄이고, 루트 노드에 대해 heapify 호출
//         print_tree(arr);
//     }
// }

fn _print_tree(arr: &[i32]) {
    let n = arr.len();
    let mut level = 0;
    let mut level_size = 1;
    let mut idx = 0;

    while idx < n {
        // Print leading spaces
        let spaces = " ".repeat((n - level_size) / 2);
        print!("{}", spaces);

        // Print nodes at the current level
        for _ in 0..level_size {
            if idx >= n {
                break;
            }
            print!("{} ", arr[idx]);
            idx += 1;
        }
        println!();

        // Print connections to the next level
        if idx < n {
            let spaces_between = " ".repeat((n - level_size) / 2 - 1);
            print!("{}", spaces_between);
            for _ in 0..level_size {
                if idx >= n {
                    break;
                }
                print!("/ \\ ");
            }
            println!();
        }

        level += 1;
        level_size *= 2;
    }
    println!("===================");
}

#[cfg(test)]
mod test {
    use crate::sorting::heap_sort::_print_tree;

    #[test]
    fn test_heapify() {
        println!("===================");
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        let len = arr.len();
        super::_heapify(&mut arr, len, 0);
        // assert_eq!(arr, vec![5, 3, 2, 4, 1]);
        _print_tree(arr.as_slice());
    }

    #[test]
    fn test_heap_sort() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        super::_heap_sort(&mut arr);
        // assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
