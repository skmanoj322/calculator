use std::io::stdin;

pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}
fn main() {
    // basic cli concept here
    // like I need the loop here to give the output here
    // right now the cargo new is only giving me nothing but the output here
    //
    //
    //

    println!("===== Rust CLI Calculator =====");

    let mut input = String::new();

    let questions = vec![
        "Enter first number:",
        "Enter operator (+, -, *, /):",
        "Enter second number:",
        "Result",
        "Do you want to calculate again? (y/n):",
    ];

    // what is worong in this code ehre
    loop {
        input.clear();

        println!("{}", questions[0]);

        let value_for_question1 = text_input(&mut input).trim().to_string();
        input.clear();

        println!("{}", questions[1]);

        let value_for_question2 = text_input(&mut input);
        input.clear();

        println!("{}", questions[2]);

        let value_for_question3 = text_input(&mut input).trim().to_string();
        input.clear();

        if value_for_question2.trim() == "+" {
            let output = parse_the_input(
                &value_for_question1,
                &value_for_question3,
                Operation::Addition,
            );
            println!("{}:{}", questions[3], output);
        } else if value_for_question2.trim() == "-" {
            let output = parse_the_input(
                &value_for_question1,
                &value_for_question3,
                Operation::Subtraction,
            );
            println!("{}:{}", questions[3], output);
        } else if value_for_question2.trim() == "*" {
            let output = parse_the_input(
                &value_for_question1,
                &value_for_question3,
                Operation::Multiplication,
            );
            println!("{}:{}", questions[3], output);
        } else if value_for_question2.trim() == "/" {
            let output = parse_the_input(
                &value_for_question1,
                &value_for_question3,
                Operation::Division,
            );
            println!("{}:{}", questions[3], output);
        }

        println!("{}", questions[4]);

        let value_for_question4 = text_input(&mut input);
        input.clear();

        if value_for_question4.trim() == "n" {
            break;
        }
    }

    println!("Good Bye");
}

fn parse_the_input(input1: &String, input2: &String, operation: Operation) -> usize {
    let val1 = input1.parse::<u32>().unwrap();
    let val2 = input2.parse::<u32>().unwrap();

    let val = match operation {
        Operation::Addition => val1 + val2,
        Operation::Subtraction => val1 - val2,
        Operation::Division => val1 / val2,
        Operation::Multiplication => val1 * val2,
    };

    return val as usize;
}

fn text_input(input: &mut String) -> String {
    match stdin().read_line(input) {
        Ok(_str) => {
            return input.to_string();
        }
        Err(error) => return format!("error: {error}"),
    }
}
