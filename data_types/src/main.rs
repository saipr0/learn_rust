fn main() {
    // *****************
    // Scalar Types: represents a single value
    // *****************

    // Integer
    let a: i8 = -3;
    let b: u32 = 3;
    let c = 57u8;
    let d = 1_00_000;
    println!("{a}, {b}, {c}, {d}");

    // floating point
    let a = 2.0;
    println!("{a}");

    // boolean
    let t = true;
    let f: bool = false;
    println!("{t}, {f}");

    // character
    let c = 'z';
    let z: char = 'ℤ';
    let robot = '🤖';
    println!("{c}, {z}, {robot}");

    // *****************
    // Compound Types: group multiple values into one type
    // *****************

    // tuples
    let tup = (500, 6.4, -2);
    let (_x, y, _z) = tup;
    let i = tup.2;
    println!("y is {y}, i is {i}");

    // arrays
    let _a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [0; 5];
    let element = a[0];
    println!("{element}")

}
