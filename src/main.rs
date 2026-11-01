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

        // had to add the mut here
        let mut value_for_question1 = text_input(&mut input).trim().to_string();
        input.clear();

        println!("{}", questions[1]);

        let mut value_for_question2 = text_input(&mut input);
        input.clear();

        println!("{}", questions[2]);

        let mut value_for_question3 = text_input(&mut input).trim().to_string();
        input.clear();

        loop {
            if value_for_question2.trim() == "+" {
                let output = parse_the_input(
                    &mut value_for_question1,
                    &mut value_for_question3,
                    Operation::Addition,
                );
                println!("{}:{}", questions[3], output);
                break;
            } else if value_for_question2.trim() == "-" {
                let output = parse_the_input(
                    &mut value_for_question1,
                    &mut value_for_question3,
                    Operation::Subtraction,
                );
                println!("{}:{}", questions[3], output);
                break;
            } else if value_for_question2.trim() == "*" {
                let output = parse_the_input(
                    &mut value_for_question1,
                    &mut value_for_question3,
                    Operation::Multiplication,
                );
                println!("{}:{}", questions[3], output);
                break;
            } else if value_for_question2.trim() == "/" {
                if get_number(&mut value_for_question3) == 0 {
                    println!("Please Not Enter the 0 in the denominator here"); // validation to check the denominator not to be zero here
                    value_for_question3 = text_input(&mut input);
                }
                let output = parse_the_input(
                    &mut value_for_question1,
                    &mut value_for_question3,
                    Operation::Division,
                );
                println!("{}:{}", questions[3], output);
                break;
            } else {
                println!("Please enter the valid input here");

                value_for_question2 = text_input(&mut input);
            }
        }

        println!("{}", questions[4]);

        let mut value_for_question4 = text_input(&mut input);
        loop {
            if value_for_question4.trim() != "y"
                || value_for_question4.trim() != "Y"
                || value_for_question4.trim() != "n"
                || value_for_question4.trim() != "y"
            {
                // this validates the invalid char here other then y or n
                println!("Only y,n are allowed please re-enter the value");
                value_for_question4 = text_input(&mut input);
            } else {
                break;
            }
        }

        if (value_for_question4.trim() == "n") || (value_for_question4.trim() == "N") {
            break; // this is the main loop here
        }
    }

    println!("Good Bye");
}
// for the get number here
fn get_number(num: &mut str) -> i32 {
    let parse = num.parse::<i32>();
    match parse {
        Ok(i) => return i,
        Err(err) => {
            println!("Error:{}", err);
            println!("Please enter the new Number here:");
            let mut input = String::new();
            let mut re_enter_input = text_input(&mut input);
            let num = get_number(&mut re_enter_input);
            return num;
        }
    };
}
//

fn parse_the_input(input1: &mut String, input2: &mut String, operation: Operation) -> i32 {
    let val1 = get_number(input1);
    let val2 = get_number(input2);

    let val = match operation {
        Operation::Addition => val1 + val2,
        Operation::Subtraction => val1 - val2,
        Operation::Division => val1 / val2,
        Operation::Multiplication => val1 * val2,
    };

    return val;
}

fn text_input(input: &mut String) -> String {
    match stdin().read_line(input) {
        Ok(_str) => {
            return input.to_string();
        }
        Err(error) => return format!("error: {error}"),
    }
}
