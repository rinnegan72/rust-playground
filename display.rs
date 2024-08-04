use std::fmt;
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}
// this is the display function for the Person structure
// It needs a life time specified for memor allocation 
// '_ means anounymous lifetime
impl fmt::Display for Person<'_> {
    // &self refers to Person instance being formatted
    // &mut means the Fmt::Fomatter is being used mutably 
    // (rust functions are immutable by default)
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {} \nAge: {}", self.name, self.age)
    }
}
fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    //printing with display
    println!("Display\n{}", peter);
    //printing with debug
    println!("Debug\n{:#?}", peter);
}