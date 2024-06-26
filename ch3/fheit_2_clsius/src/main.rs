use std::io;
use std::io::Write;

fn c_to_f(c:f64) -> String{
    let f:f64=(c-32.0)*(5.0/9.0);
    format!("in Cº is {f}Fº")
}
fn f_to_c(f:f64) -> String{
    let f:f64=f*(9.0/5.0)-32.0;
    format!("in Cº is {f}Fº")
}

fn main() {
    let mut input = String::new();

    // Prompt for conversion type
    print!("Celsius to Fahrenheit [C], or Fahrenheit to Celsius [F]: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately
    io::stdin().read_line(&mut input).expect("Unable to read input");

    let conversion_type = input.trim().to_uppercase(); // Handle case sensitivity

    // Clear the input string to reuse it for the temperature
    input.clear();

    // Prompt for temperature
    print!("Write a temperature in {}: ", conversion_type);
    io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately
    io::stdin().read_line(&mut input).expect("Unable to read input");

    let temperature: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a number, returning default value 0.0");
            0.0
        },
    };

    let result = if conversion_type == "C" {
        c_to_f(temperature)
    } else if conversion_type == "F" {
        f_to_c(temperature)
    } else {
        String::from("Invalid conversion type")
    };

    println!("{}", result);
}