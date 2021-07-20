use std::io;

fn main() {
    println!("This will generate nth fibonacci digit");

    let mut number = String::new();

    io::stdin().read_line (&mut number)
         .expect("failed to read line");

    let n: i32 = match number.trim().parse() { 
        Ok(n) => n,
        Err(e) => {
            println!("Error: {}",e);
            0
        },
    };

    // println!("The result is {}", 1 * x = 1 * x-1 + 1 * x-2);

    println!("fib number is: {}", fib(n));
}

fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),    
    }
}

#[test]
fn test_fib0(){
    let f = fib(0);
    assert_eq!(f,0);
}

#[test]
fn test_fib1() {
    let f = fib(1);
    assert_eq!(f,1)
}