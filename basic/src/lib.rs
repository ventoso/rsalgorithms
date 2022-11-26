// bubble_sort function defination this position
pub fn bubble_sort<T: Ord + std::fmt::Debug>(list: &mut [T]) {
    let len = list.len();
    for p in 0..len {
        for i in 0..len - 1 - p {
            // println!("{:?}", list);
            if list[i] > list[i + 1] {
                list.swap(i, i + 1)
            }
        }
    }
}

// selection_sort function defination this position
pub fn selection_sort<T>(list: &mut [T])
where
    T: Ord + std::fmt::Debug,
{
    let len = list.len();
    for left_index in 0..len {
        let mut small_index = left_index;
        for right_index in (left_index + 1)..len {
            if list[small_index] > list[right_index] {
                small_index = right_index;
            }
        }
        list.swap(small_index, left_index);
        println!("{:?}", list);
    }
}

// insert_sort function defination here
pub fn insert_sort<T: Ord + std::fmt::Debug>(list: &mut [T]) {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn it_works() {
        let mut list = vec![1, 20, 50, 0, 2];
        bubble_sort(&mut list);
        assert_eq!(list, vec![0, 1, 2, 20, 50]);
    }

    #[test]

    fn selection() {
        let mut list = vec![1, 20, 50, 0, 2];
        selection_sort(&mut list);
        for i in 0..list.len() - 1 {
            assert!(list[i] <= list[i + 1]);
        }
    }
}
