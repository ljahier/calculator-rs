use std::env;

mod operations;

use crate::operations::{add, divide};

fn main() {
    let cliArgs: Vec<String> = env::args().collect();

    // for (i, arg) in cliArgs.iter().enumerate() {
    //     // println!("{} = {}", &arg, i);
    // }
    let operator = &cliArgs[2];

    if operator == "+" {
        let addition = add::Add::new(cliArgs[1].parse().unwrap(), cliArgs[3].parse().unwrap());

        println!("{}", add::Add::compute(&addition));
    } else if operator == "-" {
        println!("Not implemented");
    } else if operator == "/" {
        println!("Not implemented");
    } else if operator == "*" {
        println!("Not implemented");
    } else {
        println!("Not implemented");
    }
}
