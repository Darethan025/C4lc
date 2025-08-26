#[derive(Debug)]

/// A struct containing basic math operators.
///
/// # Examples
///
/// ```
///use C4lc::Operators;

///fn main() {
///    let ops = Operators::new();
///    
///    println!("Type your number to be added\n");
///    
///    let mut input = String::new();
///    std::io::stdin().read_line(&mut input).expect("Failed to read input");
///    
///    let input: f64 = match input.trim().parse() {
///        Ok(num) => num,
///        Err(_) => {
///            println!("Not a valid input!");
///            return;
///        }
///    };
///    
///    println!("\nType your second number\n");
///    let mut input2 = String::new();
///    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
///    
///    let input2: f64 = match input2.trim().parse() {
///        Ok(num) => num,
///        Err(_) => {
///            println!("Not a valid input!");
///            return;
///        }
///    };
///    
///    let answer = match ops.addition {
///       '+' => input + input2,  // matches the char and if the wrong symbol is used, the result will be wrong
///        _ => input,
///    };
///
///    println!("\n{} + {} is: {}", input, input2, answer);
/// }
/// ```


pub struct Operators {
    pub addition: char,
    pub subtraction: char,
    pub multiplication: char,
    pub division: char,
    pub modulo: char,
}

/// There's a struct declared with the required fields to be accessed.
/// They're public and can be accessed in project directory if the crate is added to your project.
/// Only the symbols defined are the ones tied to the fields.

impl Operators {
    pub fn new() -> Self {
        Self {
            addition: '+',
            subtraction: '-',
            multiplication: '*',
            division: '/',
            modulo: '%',
        }
    }
}