#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}
//structures like above can't be printed due to rust's type safety
//the derive debug line alows us to print stuff that is not normally allowed to be printed for debugging purposes
fn main() {
    let name = "Peter Parker";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
    // Prints in 1 line
    println!("{:?}", peter);
}