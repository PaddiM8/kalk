use ansi_term::Colour::Red;
use kalk::parser::{self};

pub fn eval(parser: &mut parser::Context, input: &str) {
    match parser::parse(parser, input, 53) {
        Ok(Some(result)) => {
            let (_, digits, exp_option) = result.to_sign_string_exp(10, None);
            let exp = if let Some(exp) = exp_option { exp } else { 0 };

            if result.is_infinite() {
                err("Too big to process.");
            /*} else if result.clone().fract() == 0 {
            println!("{}", result.to_integer().unwrap());*/
            } else {
                let use_sci_notation = exp > 8 || exp < -6;
                let comma_pos = if use_sci_notation { 1 } else { exp as usize };
                let sign = if result >= 0 { "" } else { "-" };

                let num = if use_sci_notation {
                    // Insert the comma if there are supposed to be decimals.
                    let mut chars: Vec<char> = digits.trim_end_matches('0').chars().collect();
                    chars.insert(comma_pos, '.');
                    chars.into_iter().collect::<String>()
                } else if exp < 0 {
                    // 0 < x < 1
                    format!("0.{}{}", "0".repeat(exp.abs() as usize), digits)
                } else {
                    // Regular number
                    digits[..(exp as usize)].to_string()
                };

                if use_sci_notation {
                    println!("{}{}*10^{}", sign, num, exp);
                } else {
                    println!("{}{}", sign, num);
                }
            }
        }
        Ok(None) => print!(""),
        Err(msg) => err(&msg),
    }
}

fn err(msg: &str) {
    println!("{}", Red.paint(msg));
}