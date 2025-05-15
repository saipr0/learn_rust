fn main() {
    let n = 6;
    if n % 2 == 0 {
        println!("{n} is divisible by 2")
    } else {
        println!("{n} is not divisible by 2")
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // values of each arm should be of same type
    println!("number is {number}");

    // Loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count up = {count}");
        let mut count_down = 3;
        loop {
            if count == 3 {
                break 'counting_up;
            }
            if count_down == 0 {
                break;
            }
            println!("count down = {count_down}");
            count_down -= 1;
        }
        count += 1;
    }

    // while

    let mut number = 3;
    while number != 0 {
        println!("{number}..");
        number -= 1;
    }
    
    println!("LIFT OFF!!");
    let a = [29, 43];
    for element in a {
        println!("The value is {element}")
    }

    for number in (1..4).rev() {
        println!("{number}..")
    }
}
