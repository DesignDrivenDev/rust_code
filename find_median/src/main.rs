fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    let mid_index = len / 2;

    if len % 2 == 0 {
        // If the length of the array is even, return the average of the two middle elements
        let mid_value1 = arr[mid_index - 1];
        let mid_value2 = arr[mid_index];
        return (mid_value1 + mid_value2) as f64 / 2.0;
    } else {
        // If the length of the array is odd, return the middle element
        return arr[mid_index] as f64;
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("The median of the array is: {}", find_median(&arr));
}
