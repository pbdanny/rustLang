use std::io;

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let x = 2.0;
    let y: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 98.5 - 4.3;
    let product = 4*30;
    let quotient = 5.67 / 3.2;
    let truncated = -5 / 3;
    let remainder = 43 % 4;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let croissant: char = '🥐';
    
    // Compound type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {x}, {y} and {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // Array, mutable & fixed size
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    println!("a value is {a:?}");

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let secont = a[1];

    println!("Please enter array index");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed ot read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not number");

    let element = a[index];

    println!("The value of element at idx {index} is: {element}");
}
