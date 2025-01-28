use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y*2;
        println!("The value of y is: {y}");
    }
    println!("The value of y is: {y}");

    // You can change the type of the variable and reuse the same name (Shadowing)
    let spaces = "   ";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    const NAME: &str = "Yazan";
    const AGE: u32 = 19; // u (Unsigned); i (Signed)   
    // And so on...

    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("The value of quotient is: {quotient}");
    println!("The value of truncated is: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");

    // Tuple can contain multiple data types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    println!("The value of x is: {x}");
    println!("The value of z is: {z}");

    // Or you can access the tuple elements directly
    let five_hundred = tup.0;   // Tuple name followed by a period and the index of the value we want to access
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    // Array must have the same data type
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("The value of first is: {first}");

    // Or you can predefine the array size and type
    let b: [i32; 5] = [1, 2, 3, 4, 5];  // Array of 5 signed 32 bit integers
    
    // Or you can initialize the array with the same value
    let c = [3; 5]; // Array of 5 elements with the value 3

    // Taking input from user, and invalid array acceess (Try using an invalid index)
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    print_my_name();

    println!("Hello world");
    println!("HIIII");
    let n = 90;
    println!(90+4);
}

fn print_my_name(){
    println!("Please enter your name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name: String = name
        .trim()
        .parse()
        .expect("Name entered was not a string");
    println!("Hello, {name}");
}