fn main(){
    println!("Hello world!");

    // variable in rust
    let x = 43;
    println!("This is the value of x : {x}");
    // note that var are immutable by default in rust
    // x = 54 ; // Error
    // we can also user __ so ligibility
    let y = 1__0________0;
    println!("y:{y}"); // will print 100

    // arithmetic
    let result = interproduct(3,7,9);
    println!("result: {result}");
}

fn interproduct(a: i32, b:i32, c:i32) -> i32{
    return a * b + b * c + a * c;
}