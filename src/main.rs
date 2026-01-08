use std::u128;


fn main() {
    println!("Hello, world!");
    println!("solution {}",solution(33));
    luck_check("683179");
    assert_eq!(true,alphanumeric("f"));
    println!("result0: {}",sum_odd_square(897000));
    println!("result1: {}",solution(1000));
    println!("result2: {}",fibonacci(100));
}



fn solution(num: i32) -> i32 {
    (1..=(num-1)).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

//1- we receive a ticket number as &str
//2- check if ticket is empty and return false if it is.
//3- check if it's a all digit vector
//4- we have to match ticket len :: lenght%2 == 0 => even :: _ => odd    
//5- for even option, we've to split off half [len/2] and store both parts in two diferent vector. Then sum these elements and finally compare both result.
fn luck_check(ticket: &str) -> Option<bool> {

    if ticket.is_empty() || !ticket.chars().all(|c| c.is_numeric()) {
        return Some(false);
    };
    
    let vec: Vec<i32> = ticket.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let mid = vec.len()/2;
    

    let (sum_left,sum_right) = match ticket.len() % 2 {
        0 => {
            let sum_left: i32 = vec[..mid].iter().sum();
            let sum_right: i32 = vec[mid..].iter().sum();
            (sum_left,sum_right)
        },
        _ =>{
            let sum_left: i32 = vec[..mid].iter().sum();
            let sum_right: i32 = vec[mid + 1..].iter().sum();
            (sum_left,sum_right)
            
        },
    };
    
    Some(sum_left == sum_right)
}

// In this example you have to validate if a user input string is alphanumeric. The given string is not nil/null/NULL/None, so you don't have to check that.

// The string has the following conditions to be alphanumeric:

// At least one character ("" is not valid)
// Allowed characters are uppercase / lowercase latin letters and digits from 0 to 9
// No whitespaces / underscore
fn alphanumeric(password: &str) -> bool {
    !password.is_empty() && password.chars().all(|c|c.is_alphanumeric())
}

// fn sum_odd_square(num:u128)->u128{
//     let sqrt = num.isqrt();
//     (1..=sqrt).filter(|x| x%2!=0).map(|x| x*x).sum()
// }

// fn sum_odd_square(num:u128)->u128{
//     let mut vec_numbers: Vec<u128> = Vec::new();
    
//     let mut count = 1;
//     loop {
//         if vec_numbers.len() == (num as usize){break ;}
//         vec_numbers.push(count*count);
//         count+=1;

//     };
//     //println!("{:?}",vec_numbers);
//     let result: u128 = vec_numbers.iter().filter(|x| *x%2!=0).sum();
//     result
// }

fn sum_odd_square(num:u128)->u128{
    (1..=num).filter(|x| x % 2 != 0).map(|x| x * x).sum()
}

fn fibonacci(num: i32)->u64{
    if num == 0 || num == 1 {return 1;}
    let sqrt_5 = (5 as f64).sqrt();
    let term1:f64 = 1./sqrt_5;
    let term2:f64 = (1.+sqrt_5)/2.;
    let term3:f64 = (1.-sqrt_5)/2.;
    (term1*(term2.powi(num)-term3.powi(num))) as u64
}

fn fibonacci(num: i32){
    let cache_mem Vec<i128> = Vec::new();
    ()
}