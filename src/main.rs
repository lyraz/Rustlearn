fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

fn main() {
    let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}fn main() {
    println!("Hello, world!");
}
