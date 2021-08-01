use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // floating point type
    let x = 2.0;
    let y: f32 = 3.0;
    println!("{}, {}", x, y);

    // numeric operations
    let sum = 5 + 10; // addtion
    let sub = 95.5 - 4.3; // substract
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    let remainder = 43 % 5; // remainder

    println!(
        "sum: {}\nsub: {}\nproduct: {}\nquotient: {}\nremainder: {}",
        sum, sub, product, quotient, remainder
    );

    // boolean type
    let t = true;
    let f: bool = false;
    println!("t: {}", t);
    println!("f: {}", f);

    // character type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat);

    // compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: {:?}", tup);
    println!("tup: {:#?}", tup);

    let (_, y, _) = tup;
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred: {}", five_hundred);
    println!("six_point_four: {}", six_point_four);
    println!("one: {}", one);

    // Array type
    // let a = [1, 2, 3, 4, 5];
    // let months = [
    //     "January",
    //     "February",
    //     "March",
    //     "April",
    //     "May",
    //     "June",
    //     "July",
    //     "August",
    //     "September",
    //     "October",
    //     "November",
    //     "December",
    // ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("{:?}", a);
    println!("{}", first);
    println!("{}", second);

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

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
