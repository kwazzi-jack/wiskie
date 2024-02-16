// examples/basic_usage.rs

// Import necessary modules from your library
use wiskie::parser::parse_expr; // Adjust the path according to your actual module structure

fn main() {
    let input_expression = "3 + 2 * (1 + 1)";
    match parse_expr(input_expression) {
        Ok((remaining, parsed_expr)) => {
            println!("Parsed Expression: {:?}", parsed_expr);
            println!("Remaining input: {:?}", remaining);
        },
        Err(error) => {
            println!("Error parsing expression: {:?}", error);
        },
    }
}
