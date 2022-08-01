fn main() {
    let mut x = 5;
    println!("x is {x}");
    x = x * x;
    println!("x is now {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("three hours is {THREE_HOURS_IN_SECONDS} seconds");
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

}
