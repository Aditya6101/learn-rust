fn main() {
    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // reassignnig
    let mut x = 5;

    println!("value of x: {x}");

    x = 6;

    println!("value of x: {x}");

    println!("value of THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let num = 10;
    let num = num + 10;

    {
        let num = num + 20;
        println!("inner scope: {num}") // should be 40
    }

    println!("global scope: {num}") // should be 20
}
