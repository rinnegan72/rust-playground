//like the import stdio in c
use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num = &self.0;
        write!(f, "[")?;
        for (count, i) in num.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", i)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let list = List(vec![1, 2, 3, 4, 5]);
    println!("{}", list);
}