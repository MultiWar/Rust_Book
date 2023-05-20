fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let n = 50;
    let fib_of_n = tabulation_fibonacci(n);

    println!("The {n}th number in the fibonacci sequence is: {fib_of_n}");
}

fn tabulation_fibonacci(n: usize) -> usize {
    let mut table = vec![0; n + 1];

    table[1] = 1;

    for index in 0..n {
        let curr = table[index];

        if index + 1 <= n { table[index + 1] += curr; }
        if index + 2 <= n { table[index + 2] += curr; }

        // println!("{index}: {curr}");
    }

    return table[n];
}