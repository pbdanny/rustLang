fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

    // let x = 5;
    // let y = x;
    // println!("x = {}, y = {}", x, y);

    // let s = String::from("hello");
    // take_ownership(s);
    // let x = 5;
    // make_copy(x);

    let s1 = give_ownership();
    let s2 = String::from("Hello");
    let s3 = take_and_give_back(s2);

    let s1 = String::from("HELLO");
    let (s2, len) = calculate_length(s1);
    println!("The length of {s2} is {len}");

    let s3 = String::from("hElLo");
    let len = calculate_length_ref(&s3);
    println!("Length of {s3} is : {len}");

    let s = String::from("Steal");
    // change(&s);
    let mut s = String::from("Borrowing mutable");
    change_borrow(&mut s);

}

fn change_borrow(some_string: &mut String) {
    // Reference = mutable , error
    some_string.push_str(", world");
}


// fn change(some_string: &String) {
//     // Reference = mutable , error
//     some_string.push_str(", world");
// }

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32)  {
    println!("{}", some_integer);
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn take_and_give_back(a_string: String) -> String {
    a_string
}