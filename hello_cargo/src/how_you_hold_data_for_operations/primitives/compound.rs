
// takes an argument called pair which is a tuple then swaps the data types
pub fn reverse (pair: (i32, bool)) -> (bool, i32) {
    let(int_param , bool_param) = pair;
    (bool_param, int_param)
}
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

pub fn main(){
    // now a tuple is a collection of different types. They are constructed using parenthesis ()
    // okay i hate suffix annotation so i would be doing one with regular annotation
    // ise rust uses format specifiers 
    let long_tuple_suffix_annotation = (1u8, 2u16, 3u32, 4u64, 
        -1i8, 2i16, -3i32, -4i64,
         0.1f32, 0.2f64, 'a', true);

    println!("Long tuple first value: {}", long_tuple_suffix_annotation.0);
    println!("Long tuple second value: {}", long_tuple_suffix_annotation.1);

    let long_tuple_normal_annotation : (u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, char, bool) = (1, 2, 3, 4, -1, 2, -3, -4, 0.1, 0.2, 'c', false);
    println!("Long tuple first value: {}", long_tuple_normal_annotation.0);
    println!("Long tuple second value: {}", long_tuple_normal_annotation.1);
    
    // from now on i would be using normal annotation please i cant do regular annotations
    let tuple_of_tuples: ((u8, u16, u32), (u64, i8), i16) = ((1, 2, 2), (4, -1), -2);
    // the {:?} is used for debugging and displays the content in readable format
    // it is very convenient to print complex data structures like tuples
    println!("{:?}", tuple_of_tuples);

    // note tuples of more than 12 elements cant be printed
    
    // tuples can be reversed
    let tuple_pair: (i32, bool) = (90, false);
    // this calls the function reverse and reverses it
    println!("This tuple has been reversed {:?}", reverse(tuple_pair));
    
}