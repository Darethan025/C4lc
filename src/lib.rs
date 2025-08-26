#[derive(Debug)]

/// A struct containing basic math operators.
///
/// # Examples
///
/// ```
/// use c4lc::Operators;
///
/// fn main() {
///     let ops = Operators::new();
///     let add = ops.addition;
/// }
/// 

pub struct Operators {
    pub addition: char,
    pub subtraction: char,
    pub multiplication: char,
    pub division: char,
    pub modulo: char,
}

/// There's a struct declared with the required fields to be accessed
/// They're public and can be accessed in project directory if the crate is added to your project
/// Only the symbols defined are the ones tied to the fields

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