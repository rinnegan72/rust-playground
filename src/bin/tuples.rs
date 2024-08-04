fn reverse (pair: (&str,bool)) -> (bool,&str)
{
    let (string_param, bool_param) = pair;
    (bool_param, string_param)
}
fn main()
{
    let friend = ("luffy",true);
    let long_tuple = (12, 2.5, "batman", 1969, false);
    println!("tuple value number 0: {}", long_tuple.0);
    let tupple_ception = (("metropolis", false, 12.4),("goku",true,'g'));
    println!("tupple in a tupple {:?}", tupple_ception);
    println!("regular tupple: {:?}", friend);
    println!("reversed tupple: {:?}", reverse(friend));
}