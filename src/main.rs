use std::env::{args, Args};

fn operations(first_number: f32, operator: char, second_number: f32) -> f32{
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*'| 'x' | 'X' => first_number * second_number,
        '/' => first_number / second_number,
        '%' => first_number % second_number,
        _ => panic!("The operator is not valid, try using: '+', '-', '*', 'x', 'X', '/' or '%'"),
    }
}

fn result_format(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}

fn main() {

    let mut args:Args = args();
    let first_arg = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second_arg = args.nth(0).unwrap();

    let first_number = first_arg.parse::<f32>().unwrap();
    let second_number = second_arg.parse::<f32>().unwrap();
    
    println!("{} {} {}", first_number, operator, second_number);

    let result = operations(first_number, operator, second_number);

    println!("The result is: {}", result_format(first_number, operator, second_number, result));
    
}