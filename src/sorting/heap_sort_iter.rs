fn _heap_sort_iter(arr: &mut [i32]) {
    let heap_size = arr.len();
    // 1. Build Max Heap
    for i in (0..arr.len() / 2).rev() {
        _heapify_iter(arr, heap_size, i);
    }

    // 2. Sort
    for i in (0..arr.len()).rev() {
        arr.swap(i, 0);
        _heapify_iter(arr, heap_size, 0);
    }
}

fn _heapify_iter(arr: &mut [i32], heap_size: usize, mut root_index: usize) {
    loop {
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
            root_index = largest_index;
        } else {
            break;
        }
    }
}

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
    use crate::sorting::heap_sort_iter::_print_tree;

    #[test]
    fn test_heapify() {
        println!("===================");
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        let len = arr.len();
        super::_heapify_iter(&mut arr, len, 0);
        // assert_eq!(arr, vec![5, 3, 2, 4, 1]);
        _print_tree(arr.as_slice());
    }

    #[test]
    fn test_heap_sort() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        super::_heap_sort_iter(&mut arr);
        // assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
