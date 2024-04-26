// scalar data types


    // Integer

    // Floating points

    // Boolean

    // Character


//compound

    //tuple

    //array


fn main(){
    //integer
    let x = 5;
    let y: i32 = 10;
    let z: u32 = 15;

    //floating points
    let a = 2.0;
    let b: f32 = 3.0;

    //boolean
    let t = true;
    let f: bool = false;

    //character
    let c = 'z';
    let z = 'Z';

    //tuple - groups together a number of values.
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    //array
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    let x: [i32; 5] = [-2, 3, 2, 1, 0];

    let first = x[0];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);
    println!("The value of first is: {}", first);
    
}


