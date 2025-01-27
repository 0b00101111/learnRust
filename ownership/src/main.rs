fn bubble_sort(arr: &mut [u32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                (arr[j], arr[j + 1]) = (arr[j + 1], arr[j]);
            }
        }
    }
}


fn main() {
    let mut numbers = [5, 1, 3, 0, 2, 9, 1, 9, 8, 9, 0, 5, 2, 1, 1, 9, 5, 6];
    bubble_sort(&mut numbers);
    println!("{:?}", numbers);

    let s_1  = String::from("hello");
    let s_2 = "Hi, there.";
    let s_3 = s_2.to_string();

    println!("{}", s_1);
    println!("{}", s_3.capacity());
    println!("{:?}", s_3.as_ptr());

    let ptr = s_3.as_ptr();

    unsafe {
        println!("First byte: {}", *ptr);

        let ptr_plus1 = ptr.add(1);
        println!("{:?}", ptr_plus1);
        println!("Pointer arithmetics: {}", *ptr_plus1);
    }
}
