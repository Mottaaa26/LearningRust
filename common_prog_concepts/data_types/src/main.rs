use std::io;

fn main() {


    //SCALAR DATA_TYPES

    //float
    let _x = 2.0; //f64
    let _y: f32 = 3.5; //f32

    //boolean
    let _t = true;
    let _f: bool = false;

    //char
    let _c = 'z';
    let _z: char = 'p';

    //COMPUND DATA_TYPES

    //tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let _tup2: (u8, u8, u8, u8, u8) = (1, 2, 3, 4, 5);

    let (_x, y, _z) = tup;
    println!("The value of y is {y}!");

    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    //arrays
    let a = [1, 2, 3, 4, 5];

    let _b: [u8; 5] = [2, 3, 4, 5, 6];

    let _c = [3; 5];

    let _first = a[0];
    let _second = a[1];

    let p = [1, 2, 3, 4, 5];
    println!("Please enter an array index!");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!");

    let element = p[index];
    println!("The value of the element at index {index} is {element}!");

}
