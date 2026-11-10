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
}