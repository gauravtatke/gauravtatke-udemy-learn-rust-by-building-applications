use std::io;
use std::io::Write;

fn main() {
    print!("Enter weight on earth: ");
    io::stdout().flush().unwrap(); // stdout is line-buffered and print! does not have newline so output may not be seen on screen
    let mut weight_on_earth = String::new();
    io::stdin().read_line(&mut weight_on_earth).unwrap();
    let weight: f32 = weight_on_earth.trim().parse().unwrap();
    println!("Weight on mars: {} Kgs", calc_weight_on_mars(weight));
}

fn calc_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.71
}