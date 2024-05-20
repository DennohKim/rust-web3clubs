fn main() {

    // Infinte loop

    let mut counter = 0;

    loop {
        counter += 1;
        println!("again!");

        if counter == 5 {
            break;
        }
    }
}