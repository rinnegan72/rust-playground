fn main(){
    //different ways of printing with formatting it
    println!("My name is {}", "Slim Shady");
    println!("Hi, my name is, {0}, My name is, {1}","what","who");
    println!("you {subject}, are the pride of {subjecthometown} ", subject="Slim", subjecthometown="Shady");
    println!("Base 10:               {}",   69420);
    //characters added before the *
    println!("{x:->5}",x='*');
    println!("{x:->4}",x='*');
    println!("{x:->3}",x='*');
    println!("{x:->2}",x='*');
    println!("{x:->1}",x='*');
    //characters added after the *
    println!("{x:-<5}",x='*');
    println!("{x:-<4}",x='*');
    println!("{x:-<3}",x='*');
    println!("{x:-<2}",x='*');
    println!("{x:-<1}",x='*');
    let number = 1.0;
    let width = 4;
    println!("{number:>width$}");
}