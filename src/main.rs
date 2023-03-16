use std::io;

fn main() {
    let day = ["","Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"];

    println!("Please enter a day number.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

        let index : usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");


    let element = day[index];

    if index == 0 {
        println!("The day is not available");}
    else {
    println!("The day is {element}");
    }

     if index  == 0 {
        println!("Not Available");
    } else if index > 5 {
        println!("The day is a weekend");}
    else {
    println!("The day is a weekday");
    }

}