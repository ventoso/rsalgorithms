pub fn binary_search(arr: Vec<i32>, left: usize, right: usize, key: i32) -> i32 {
    if left > right {
        return -1;
    } else {
        let mid = left + (right - left) / 2;
        // match arr[mid] {
        //     key..1000000 => binary_search(arr, left, mid - 1, key),
        //     -1100000000..key => binary_search(arr, mid + 1, right, key),
        //     _ => mid as i32,
        // }
        match arr[mid] {
            res if res == key => mid as i32,
            res if res > key => binary_search(arr, left, mid - 1, key),
            _ => binary_search(arr, mid + 1, right, key),
        }
        // if arr[mid] == key {
        //     return mid as i32;
        // } else if arr[mid] > key {
        //     return binary_search(arr, left, mid - 1, key);
        // } else {
        //     return binary_search(arr, mid + 1, right, key);
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec = vec![1, 4, 23, 123, 12151, 12222, 1214142];
        let result = binary_search(vec, 0, 6, 23);
        assert_eq!(result, 2);
    }
}
