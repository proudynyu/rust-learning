pub fn linear_search(arr: &[i64], target: i64) -> bool {
    for x in 0..arr.len()  {
        if arr[x] == target {
            return true
        }
    }
    return false
}
