use std::io;

fn fibo(n1: i64, n2: i64) -> i64 {
    if n1 > 2 {
        return fibo(n1 - 2, n2) * n2 + fibo(n1 - 1, n2);
    } else {
        return 1;
    }
}

fn main() {
    let mut n1_string = String::new();
    let mut n2_string = String::new();

    println!("Enter the first integer, where n≤40:");
    io::stdin()
        .read_line(&mut n1_string)
        .expect("Failed to read line");
    println!("Enter the first integer, where n≤5:");
    io::stdin()
        .read_line(&mut n2_string)
        .expect("Failed to read line");

    let n1: i64 = n1_string.trim().parse::<i64>().unwrap();
    let n2: i64 = n2_string.trim().parse::<i64>().unwrap();

    let output = fibo(n1, n2);

    println!("{output}");
}
