pub fn bubble_sort(arr: mut &[usize]) -> void {
    for x in 0..arr.len() {
        for y in 0..(arr.len() - x) {
            if arr[x] > arr[x+1] {
                let tmp = arr[x];
                arr[x] = arr[x+1];
                arr[x+1] = tmp;
            }
        }
    }
}
