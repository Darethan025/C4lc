pub fn greet() {
    println!("\n\nHello, you just called the C4lc public function greet() and to use the public struct Operators, you can do this let variablename = C4lc::Operators, and add the missing fields to use the struct for your mathematical operations\n\n");
}

#[derive(Debug)]
pub struct Operators {
    pub addition: char,
    pub subtraction: char,
    pub multiplication: char,
    pub division: char,
}