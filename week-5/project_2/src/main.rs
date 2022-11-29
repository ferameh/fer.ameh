use std::io;

fn main() {
    println!("Are you experienced(1) or inexperienced(2)?\nPut 1 or 2 as indicated");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("I don't understand");
    let mut num:u32  = x.trim().parse().expect("I don't understand");

    println!("What is your age?");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("I don't understand");
    let mut num2:u32 = y.trim().parse().expect("I don't understand");

    if num == 1 && num2>=40 {
        println!("Your incentive is N1,560,000 ");
    }
    if num == 1 && num2>=30 && num2<40 {
        println!("Your incentive is N1,480,000");       
    }
    if num ==1 && num2 < 28  {
        println!("Your incentive is N1,300,000");
    }
    else if num != 1 {
        println!("Your incentive is N100,000");
    }
}