fn main() {
    // 1. Define arrays (fixed-size, elements of the same type)
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [0; 10];  // Create an array of length 10, all elements initialized to 0

    // 2. Access array elements by index (indexing starts at 0)
    println!("First element: {}", arr[0]);
    println!("Third element: {}", arr[2]);

    // 3. Get array length using .len()
    println!("Array length: {}", arr.len());

    // 4. Mutable array
    let mut mut_arr = [10, 20, 30];
    mut_arr[1] = 50;  // Modify the second element
    println!("Mutable array: {:?}", mut_arr);

    // 5. Out-of-bounds access causes runtime panic
    // println!("{}", arr[10]);  // Don't do this, will panic!

    // 6. Array slices (&[T]) - references to part of an array
    let slice: &[i32] = &arr[1..4];  // Includes elements at indices 1, 2, and 3
    println!("Slice: {:?}", slice);

    // 7. Mutable slices (&mut [T])
    let slice_mut: &mut [i32] = &mut mut_arr[0..2];
    slice_mut[0] = 100;  // Modify first element in the slice
    println!("Modified mutable slice: {:?}", slice_mut);
    println!("Modified original array: {:?}", mut_arr);

    // 8. Multi-dimensional arrays (arrays of arrays)
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    println!("Matrix element [1][2]: {}", matrix[1][2]);
}