use std::io;

fn main() {
    // Input values for experience and age
    println!("Enter years of experience:");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read line");
    let experience:i32 = experience.trim().parse().expect("Invalid input. Please enter a number.");

    println!("Enter age:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age:i32 = age.trim().parse().expect("Invalid input. Please enter a number.");

    // Determine the annual incentive based on the provided criteria
    let annual_incentive = if experience > 0 {
        if age >= 40 {
            1560000
        } else if age >= 30 {
            1480000
        } else if age >= 28 {
            1300000
        } else {
            100000
        }
    } else {
        100000
    };

    println!("The annual incentive is: N{}", annual_incentive);
}