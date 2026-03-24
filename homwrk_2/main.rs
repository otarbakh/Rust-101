use std::io;

fn main() {
    // fizz_buzz();
    // score_calculator(); 
    sale_calculator();
}

fn fizz_buzz() {
    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}




fn score_calculator() {
    println!("Enter your score (0-100):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let score: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };


    if score >= 90 && score <= 100 {
        println!("A");
    } else if score >= 80 && score <= 89 {
        println!("B");
    } else if score >= 70 && score <= 79 {
        println!("C");
    } else if score >= 60 && score <= 69 {
        println!("D");
    } else if score >= 0 && score <= 59 {
        println!("F");
    } else {
        println!("Invalid grade");
    }
}





fn sale_calculator() {
    println!("Enter product price (float):");
    let mut price_input = String::new();
    io::stdin().read_line(&mut price_input).expect("Failed to read input");
    let price: f64 = match price_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };

    println!("Enter quantity (int):");
    let mut qty_input = String::new();
    io::stdin().read_line(&mut qty_input).expect("Failed to read input");
    let quantity: i32 = match qty_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };

    println!("Do you have premium package? (true/false):");
    let mut premium_input = String::new();
    io::stdin().read_line(&mut premium_input).expect("Failed to read input");
    let premium: bool = match premium_input.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };

    
    let initial_total = price * quantity as f64;
    let mut discount = 0.0;

    
    if initial_total > 100.0 {
        discount += initial_total * 0.10;
    }


    if quantity >= 10 {
        discount += initial_total * 0.05;
    }


    let mut final_total = initial_total - discount;
    if premium {
        final_total -= 10.0;
    }

    println!("Initial total: {:.2}", initial_total);
    println!("Total discount: {:.2}", discount);
    println!("Final total: {:.2}", final_total);
}