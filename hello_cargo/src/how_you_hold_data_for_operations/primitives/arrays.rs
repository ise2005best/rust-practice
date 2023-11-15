 use std::mem;

fn analyze_slice (slice: &[i32]){
    println!("First element of the slice: {}", slice[0]);
    println!("The length of the array: {}", slice.len());
}

// & can be used when passing a function parameter and in borrowing
pub fn main(){
    // in rust you initialize an array like this let array : [dataType, lengthOfArray]
    let xs: [i32; 5] = [1,2,3,4,5];
    // this basically initializes all 90 elements in the array to 0
    let yl: [i32; 90] = [0; 90];

    println!("First element of the array: {}", xs[0]);
    println!("Number of elements in the array: {}", xs.len());

    println!("Arrays occupies {} bytes", mem::size_of_val(&yl));
    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // we can borrow a part of an array
    analyze_slice(&yl[0 .. 10]);

    let empty_array: [i32; 0] = [];
    assert_eq!(&empty_array, &[]);

}


// fn multiplier_classwork(multi: &[f64]) -> f64{
//     let mut i = 0;
//     let mut result: f64 = 1.0;
//     while i < multi.len(){
//         result *= result * multi[i];
//         i += 1.0;
//     }
//     result
// }