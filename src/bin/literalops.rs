fn main(){
    println!("1 - 2 = {}", 1i32 - 2); //making 1 here unsigned causes overflow error
    println!("One million is written as {}", 1_000_000u32); //the underscores make it more readable and u32 makes sure that it prints an integer
}