//fn main() {
// Define the number of terms in the Fibonacci sequence you want to generate
   // let n = 15; // You can change this to any positive integer

// Call the function to print the Fibonacci sequence
//     print_fibonacci(n);
// }

// fn print_fibonacci(n: usize) {
// // Initialize the first two Fibonacci numbers
//     let mut a = 0;
//     let mut b = 1;
//
// // Print the first Fibonacci number
//     if n >= 1 {
//         println!("{}", a);
//     }
//
// // Print the second Fibonacci number
//     if n >= 2 {
//         println!("{}", b);
//     }
//
// // Generate and print the rest of the Fibonacci sequence
//     for _ in 2..n {
//         let next = a + b;
//         println!("{}", next);
//         a = b;
//         b = next;
//     }
// }


// Recursion - always requires a condition


// mod recursion;
mod game_character;
mod division;

// use recursion::recursion::recursion_function;
// use game_character::game_character::GameCharacter;

use division::division::division_function;

fn main() {
    // let mut num = 4;

    // recursion_function(num);

    // let mut game_character1 = GameCharacter {
    //     name: String::from("Chizu"),
    //     score: 4300,
    //     level: 2
    // };

    // println!("game character:\n {}\n {}\n {}", game_character1.name, game_character1.score, game_character1.level);

    // game_character1.get_character_name();
    // game_character1.increase_level();

    division_function();





}

// impl GameCharacter {
//     fn get_character_name(&self) {
//         println!("character name: {}",self.name );
//     }
//
//     fn increase_level(&mut self) {
//         self.level += 1;
//         println!("character name: {}",self.level );
//
//     }
//
// }


// write a program that takes in an input, takes in a second input from terminal, divided the second input from the terminal. Test for condition: divide by zero, integer overflow -  write tests




