pub fn recursion_function (mut par :i32){

    println!("number {par}");
    if par <= 10 {
        par+=1;
        recursion_function(par);
    }
    return;
}
