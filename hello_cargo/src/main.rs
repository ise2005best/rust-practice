// mod variables;

// use std::io; // standard input, output library in rust

// fn main(){
//     println!("Guess the number!");

//     println!("Please input your guess.");

//     let mut guess = String::new(); // a muttable variable with a data type of string, attached to new as we are creating an empty instance of a string that is not attached to a value.

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {guess}")

// }
mod greetings;
mod how_you_hold_data_for_operations;
use greetings::default_greeting;
use how_you_hold_data_for_operations:: primitives;
use primitives::scalar;
use scalar::scalar_literals;

use how_you_hold_data_for_operations::derived;
// extern crate hello_world_lib;

fn main(){
    // let (mutable) =  scalar_literals();
    println!("Hello World!");
//     println!("{}", default_greeting());
//    scalar_literals();
//    primitives::compound::main();
//    primitives::literals::main();
//    primitives::arrays::main();
   derived::struct_practice::main();
   derived::struct_practice::enums();
}