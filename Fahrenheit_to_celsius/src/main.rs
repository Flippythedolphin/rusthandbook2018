use std::io;

fn main() {
    println!("This converts numbers between fahrenheit and celsius\n\n");

    let mut number = String::new();


    println!("Please enter a number in fahrenheit:");
    
    io::stdin().read_line (&mut number)
         .expect("failed to read line");

    let x: i32 = match number.trim().parse() { 
        Ok(n) => n,
        Err(e) => {
            println!("Error: {}",e);
            0
        },
    };

    println!("celsius = {}", (x - 32) * 5/9 );  
}
