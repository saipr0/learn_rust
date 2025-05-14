fn main() {
    println!("Hello, world!");
    print_x(five());
    
}

fn print_x(x: i32) {
    println!("x is {x}");
}

fn five() -> i32 {
    5
}
