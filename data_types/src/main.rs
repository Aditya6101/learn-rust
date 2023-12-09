use std::io;
fn main() {
    // scaler types

    // 1. interger types

    // signed -> explicity negative/positive symbol
    let int1: i32 = -20;

    // unsigned
    let int2: u32 = 20;

    println!("x: {int1} y: {int2}");

    // 2. floats
    let float1 = 2.6; // f64

    let float2: f32 = 3.8; // f32

    println!("float1: {float1}, float2: {float2}");

    // 3. booleans

    let boolean = true;

    println!("boolean: {boolean}");

    // 4. char

    let char1 = ')';
    let char2 = '😀';
    let char3 = 'ॶ';

    println!("char1: {char1}, char2: {char2}, char3: {char3}");

    // compund types

    // 1. tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (t1, t2, t3) = tup;

    println!("t1: {t1}, t2: {t2}, t3: {t3}");

    let idx2 = tup.1;
    println!("t2: {idx2}");

    // empty tuple -> unit
    let _empty_tup = ();

    // 2. array
    let arr1 = [1, 2, 3, 4, 5];
    let _arr2 = [6; 6];
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let birth_month = months[9];

    println!("birth_month: {birth_month}");

    // panic -> index out of bounds
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr1[index];

    println!("The value of the element at index {index} is: {element}");
}
