use std::io;

fn main() {
    println!("QUADRATIC EQUATION CALCULATOR\nax^2 +bx +c");

    let mut a:f32 = 0.0;
    let mut b:f32 = 0.0;
    let mut c:f32 = 0.0;
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut root1:f32 = 0.0;
    let mut root2:f32 = 0.0;

    println!("Input a: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    a = input1.trim().parse().expect("Failed to read");

    println!("Input b: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    b = input2.trim().parse().expect("Failed to read input");
    
    println!("Input c: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    c = input3.trim().parse().expect("Failed to understand");

    let mut discriminant:f32 = f32::powf(b,2.0) - 4.0 * a * c;
    if discriminant<0.0 {
        println!("There are no real roots");
    }
    else if discriminant == 0.0 {
        root1 = -b / 2.0 * a;
        println!("Root = {}",root1);
    }
    else if discriminant > 0.0 {
        root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Root1 = {}", root1);
        println!("Root2 = {}", root2);
    }
}