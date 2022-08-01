use std::io;

fn main() {
    let n = loop {
        println!("What Fibonacci number would you like?");

        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break n;
    };

    let mut prev = 0;
    let mut val = if n == 0 { 0 } else { 1 };
    for _ in 2..n + 1 {
        (prev, val) = (val, val + prev);
    }

    println!("Fibonacci sequence number {n} is {val}");
}
