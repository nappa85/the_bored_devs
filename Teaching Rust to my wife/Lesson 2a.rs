
//! Write a struct with 2 fields of type `u8`, implement for it a constructor and then a method to sum, substract, multiply and divide the two numbers, printing the results.
//! Then create another struct that does the same with 3 fields.

struct OperationsTwo {
    parameter1: u8,
    parameter2: u8,
}

impl OperationsTwo {
    fn new(parameter1: u8, parameter2: u8) -> Self {
        OperationsTwo { parameter1, parameter2 }
    }
    // worst case scenario 255 + 255 = 510
    fn sum(&self) -> u16 {
        u16::from(self.parameter1) + u16::from(self.parameter2)
    }
    // worst case scenario 0 - 255 = -255
    fn sub(&self) -> i16 {
        i16::from(self.parameter1) - i16::from(self.parameter2)
    }
    // worst case scenario 255 * 255 = 65025
    fn mul(&self) -> u16 {
        u16::from(self.parameter1) * u16::from(self.parameter2)
    }
    // we want float division instead of integer division
    fn div(&self) -> f64 {
        f64::from(self.parameter1) / f64::from(self.parameter2)
    }
    // remainder doesn't have limit problems
    fn rem(&self) -> u8 {
        self.parameter1 % self.parameter2
    }
}

struct OperationsThree {
    parameter1: u8,
    parameter2: u8,
    parameter3: u8,
}

impl OperationsThree {
    fn new(parameter1: u8, parameter2: u8, parameter3: u8) -> Self {
        OperationsThree { parameter1, parameter2, parameter3 }
    }
    // worst case scenario 255 + 255 + 255 = 765
    fn sum(&self) -> u16 {
        u16::from(self.parameter1) + u16::from(self.parameter2) + u16::from(self.parameter3)
    }
    // worst case scenario 0 - 255 - 255 = -510
    fn sub(&self) -> i16 {
        i16::from(self.parameter1) - i16::from(self.parameter2) - i16::from(self.parameter3)
    }
    // worst case scenario 255 * 255 * 255 = 16581375
    fn mul(&self) -> u32 {
        u32::from(self.parameter1) * u32::from(self.parameter2) * u32::from(self.parameter3)
    }
    // we want float division instead of integer division
    fn div(&self) -> f64 {
        f64::from(self.parameter1) / f64::from(self.parameter2) / f64::from(self.parameter3)
    }
    // remainder doesn't have limit problems
    fn rem(&self) -> u8 {
        self.parameter1 % self.parameter2 % self.parameter3
    }
}

fn main() {
    let a = 17;
    let b = 24;
    let c = OperationsTwo::new(a, b);
    println!("{a} + {b} = {}", c.sum());
    println!("{a} - {b} = {}", c.sub());
    println!("{a} * {b} = {}", c.mul());
    println!("{a} / {b} = {}", c.div());
    println!("{a} % {b} = {}", c.rem());

    let a = 17;
    let b = 24;
    let c = 56;
    let d = OperationsThree::new(a, b, c);
    println!("{a} + {b} + {c} = {}", d.sum());
    println!("{a} - {b} - {c} = {}", d.sub());
    println!("{a} * {b} * {c} = {}", d.mul());
    println!("{a} / {b} / {c} = {}", d.div());
    println!("{a} % {b} % {c} = {}", d.rem());
}
