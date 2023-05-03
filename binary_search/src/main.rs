fn binary_search(arr: &[u32], needle: &u32) -> Option<usize> {
    let mut low: u32 = 0;
    let mut high: u32 = arr.len() as u32;

    while low <= high {
        let middle: u32 = low + ((high - low)/2);
        let idx = middle as usize;
        let target = &arr[idx];

        if target == needle {
            return Some(idx);
        }

        if target > needle {
            high = middle;
        }

        if target < needle {
            low = middle + 1;
        }
    }
    None
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8];
    let target: u32 = 4;
    let idx = binary_search(&arr, &target);

    println!("{:?}", idx);
}
