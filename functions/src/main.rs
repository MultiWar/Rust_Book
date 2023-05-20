fn main() {
    println!("Hello, world!");

    print_labeled_measurement(5, 'h');

    another_function(25);

    let (y, z) = (test_using_return(), test_without_return());

    println!("Y is: {y} and Z is: {z}");
}

fn another_function(x: i32) {
    println!("Another function, executed with the argument: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn test_using_return() -> u32 {
    return 7;
}

fn test_without_return() -> u32 {
    // note the lack of ; at the end of the line.
    // putting one would make it a statement instead of an expression,
    // so no value would be returned.
    5
}