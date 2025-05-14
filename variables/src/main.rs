fn main() {
    const HOURS_IN_WEEK: u32 = 7 * 24;
    println!("There are {HOURS_IN_WEEK} hours in a week");

    // mutable variable
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is {x}");

    // shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("y is {y}");
    }
    println!("y is {y}");
}
