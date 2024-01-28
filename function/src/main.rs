// this is how we add comments
fn main() {
    println!("Hello, world!");
    cool_function();
    function();
    let output_of_five = five();
    println!("{output_of_five}");
}

fn cool_function() {
    println!("this is a cool function");
}

fn five() -> i32 {
    5
}

fn function() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
