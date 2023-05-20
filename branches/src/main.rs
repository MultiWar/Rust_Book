fn main() {
    // THIS SHIT WORKS, BECAUSE X IS ONLY IMMUTABLE *AFTER HAVING A VALUE ASSIGNED TO IT*
    let x;

    if false {
        x = 1;
    } else {
        x = 2;
    }

    println!("X: {x}");
}
