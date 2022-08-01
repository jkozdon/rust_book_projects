fn main() {
    println!("Hello, world!");

    another_function(fives().0);
}

fn another_function(x : i32) {
    println!("Another function. {x}");
}

fn fives() -> (i32, i32) {
    (5, 5)
}
