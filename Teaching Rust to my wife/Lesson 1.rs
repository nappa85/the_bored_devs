
//! Write a function for every possible operation between two `u8`.
//! Keep in mind that certain operations can violate the boundaries of the type, manage it as needed.

// worst case scenario 255 + 255 = 510
fn sum(parameter1: u8, parameter2: u8) -> u16 {
    u16::from(parameter1) + u16::from(parameter2)
}
// worst case scenario 0 - 255 = -255
fn sub(parameter1: u8, parameter2: u8) -> i16 {
    i16::from(parameter1) - i16::from(parameter2)
}
// worst case scenario 255 * 255 = 65025
fn mul(parameter1: u8, parameter2: u8) -> u16 {
    u16::from(parameter1) * u16::from(parameter2)
}
// we want float division instead of integer division
fn div(parameter1: u8, parameter2: u8) -> f64 {
    f64::from(parameter1) / f64::from(parameter2)
}
// remainder doesn't have limit problems
fn rem(parameter1: u8, parameter2: u8) -> u8 {
    parameter1 % parameter2
}

fn main() {
    let a = 17;
    let b = 24;
    println!("{a} + {b} = {}", sum(a, b));
    println!("{a} - {b} = {}", sub(a, b));
    println!("{a} * {b} = {}", mul(a, b));
    println!("{a} / {b} = {}", div(a, b));
    println!("{a} % {b} = {}", rem(a, b));
}
