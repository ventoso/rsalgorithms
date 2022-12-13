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
    // get the length of the array
    let len = list.len();
    // 从0开始便利数组到最后一位减一
    //（因为当除最后一位都通过选择排序排序之后，最后一位也顺理成章的时最后一位了）
    for left_index in 0..(len - 1) {
        // get the origin index equal left_index
        let mut small_index = left_index;
        // 遍历数组找到剩下无序数列中最小的值的索引
        for right_index in (left_index + 1)..len {
            if list[small_index] > list[right_index] {
                small_index = right_index;
            }
        }
        // exchange small_index to left_index
        // complete once selection
        list.swap(small_index, left_index);
        println!("{:?}", list);
    }
}

// insert_sort function defination here
pub fn insert_sort<T>(list: &[T]) -> Vec<T>
where
    T: Ord + Clone + std::fmt::Debug,
{
    // build a new vector which have the same capacity with inputed list
    // can save many memory request 开销
    let mut res: Vec<T> = Vec::with_capacity(list.len());
    // 调用.iter().cloned()方法返回一个cloned的拥有ownership的元素迭代器
    for item in list.iter().cloned() {
        // get the length of res
        // the n_len is improving
        let n_len = res.len();
        // 从头遍历有序数组res
        // 如果有序数组有比item大的元素，则将item插入到那个元素前面
        // 如果遍历结束，即 i == n_len,代表有序数组的每个元素都要小
        // 于 item ，那么插入到有序数组最后
        for i in 0..=n_len {
            if i == n_len || res[i] > item {
                res.insert(i, item);
                // println!("{:?}", res);
                break;
            }
        }
    }
    res
}
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
        let mut list = vec![10, 20, 50, 0, 2];
        selection_sort(&mut list);
        for i in 0..list.len() - 1 {
            assert!(list[i] <= list[i + 1]);
        }
    }

    #[test]
    fn insert() {
        let list = vec![1, 20, 2, 24, 20, 3];
        let list = insert_sort(&list);
        println!("{:?}", list);
        for i in 0..list.len() - 1 {
            assert!(list[i] <= list[i + 1]);
        }
    }
}
