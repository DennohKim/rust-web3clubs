// write a program that takes in an input, takes in a second input from terminal, divided the second input from the terminal. Test for condition: divide by zero, integer overflow -  write tests

//get inputs from terminal

// perform division

// Add check for when the number is divided by 0

use std::io;

pub fn division_function(){

    //Enter first Number
    println!("Enter the first number");

    let mut number1 = String::new();

    io::stdin().read_line(&mut number1).expect("Failed to read  line");
    let number1: u32 = number1.trim().parse().expect("Please enter a number!");

    println!("Your first number is: {number1}");


    //Enter Second Number
    println!("Enter the second number");

    let mut number2 = String::new();

    io::stdin().read_line(&mut number2).expect("Failed to read  line");
    let number2: u32 = number2.trim().parse().expect("Please enter a number!");

    println!("Your Second number is: {number2}");

    //Perform division

    if number1 == 0 {
        panic!("Zero Division Error")

    } else {
        let mut division_result = number1/number2;
        println!("Division Result: {division_result}");


    }









}