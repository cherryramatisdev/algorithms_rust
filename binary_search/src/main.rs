fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut smallest = 0;
    let mut highest = arr.len() - 1;

    if target.is_negative() {
        return None;
    }

    while smallest <= highest {
        let middle = (smallest + highest) / 2;
        let guess = arr[middle];

        if guess == target {
            return Some(middle);
        } else if guess > target {
            highest = middle - 1;
        } else {
            smallest = middle + 1;
        }
    }

    None
}
fn main() {
    let list = vec![1, 3, 5, 7, 9];

    println!("binary_search(&list, 3) :: {:?}", binary_search(&list, 3));
    println!("binary_search(&list, -1) :: {:?}", binary_search(&list, -1));
}
