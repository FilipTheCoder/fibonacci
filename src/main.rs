use std::fmt::Display;






fn main() {

    println!("Hello, world!");
    println!("Input the length of fibonacci sec");
    let input = get_number_from_user();

    let fib = fibonacci_sec(input);
    print_vec(&fib)
}





fn get_number_from_user() -> i32 {
    let result: i32;
    
    loop {
        let mut input_string = String::new();
        match std::io::stdin().read_line(&mut input_string) {
            Ok(_i) => {
                println!("Input was successfully accepted");
                
            },
            Err(err) => {
                println!("There was an error: \n{}", err.to_string());
                println!("Try again");
            }
        };
    
        match input_string.trim().parse::<i32>() {
            Ok(n) => {
                result = n;
                break;
            },
            Err(err) => {
                println!("There was an error: {}", err.to_string());
                println!("Try again");
                continue;
                
            }
    
        };
        
    }



    return result
}




fn print_vec<T>(arr: &Vec<T>)
where T: Display
{
    for item_to_print in arr {
        println!("{}", item_to_print)
    }
}



fn fibonacci_sec(n: i32) -> Vec<i32> {
    let mut a = 0;
    let mut b = 1;
    let mut result = Vec::new();
    for _i in 0..n {
        let temp = a+b;
        a = b;
        b = temp;
        result.push(temp);
    }
    return result
}