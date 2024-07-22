fn main(){
    // 1. print hello world
    println!("Hello World");

    // 2.  normal calculation
    let x = 5 + 90 + 5;
    let y =x +1 ;
    println!("Is `x`  10 or 100 ? x = {}, {}", x, y);

    // 3. fomatted prints
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Maulik", "Dave");

    // As can named arguments.
    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    
}