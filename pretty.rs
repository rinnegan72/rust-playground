fn main(){
    println!("My name is {}", "Slim Shady");
    println!("Hi, my name is, {0}, My name is, {1}","what","who");
    println!("you {subject}, are the pride of {subjecthometown} ", subject="Slim", subjecthometown="Shady");
    println!("Base 10:               {}",   69420);
    println!("{x:->5}",x='*');
    println!("{x:->4}",x='*');
    println!("{x:->3}",x='*');
    println!("{x:->2}",x='*');
    println!("{x:->1}",x='*');
    println!("{x:-<5}",x='*');
    println!("{x:-<4}",x='*');
    println!("{x:-<3}",x='*');
    println!("{x:-<2}",x='*');
    println!("{x:-<1}",x='*');
    let number = 1.0;
    let width = 4;
    println!("{number:>width$}");
}