
//! Create an `Operations` trait that has a method for every operation between numbers, then convert structs from previous homework about structs into using the new trait.

trait Operations {
    type SumOutput;
    fn sum(&self) -> Self::SumOutput;
    type SubOutput;
    fn sub(&self) -> Self::SubOutput;
    type MulOutput;
    fn mul(&self) -> Self::MulOutput;
    type DivOutput;
    fn div(&self) -> Self::DivOutput;
    type RemOutput;
    fn rem(&self) -> Self::RemOutput;
}

struct OperationsTwo {
    parameter1: u8,
    parameter2: u8,
}

impl OperationsTwo {
    fn new(parameter1: u8, parameter2: u8) -> Self {
        OperationsTwo { parameter1, parameter2 }
    }
}

impl Operations for OperationsTwo {
    // worst case scenario 255 + 255 = 510
    type SumOutput = u16;
    fn sum(&self) -> Self::SumOutput {
        Self::SumOutput::from(self.parameter1) + Self::SumOutput::from(self.parameter2)
    }
    // worst case scenario 0 - 255 = -255
    type SubOutput = i16;
    fn sub(&self) -> Self::SubOutput {
        Self::SubOutput::from(self.parameter1) - Self::SubOutput::from(self.parameter2)
    }
    // worst case scenario 255 * 255 = 65025
    type MulOutput = u16;
    fn mul(&self) -> Self::MulOutput {
        Self::MulOutput::from(self.parameter1) * Self::MulOutput::from(self.parameter2)
    }
    // we want float division instead of integer division
    type DivOutput = f64;
    fn div(&self) -> Self::DivOutput {
        Self::DivOutput::from(self.parameter1) / Self::DivOutput::from(self.parameter2)
    }
    // remainder doesn't have limit problems
    type RemOutput = u8;
    fn rem(&self) -> Self::RemOutput {
        Self::RemOutput::from(self.parameter1) % Self::RemOutput::from(self.parameter2)
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
}

impl Operations for OperationsThree {
    // worst case scenario 255 + 255 + 255 = 765
    type SumOutput = u16;
    fn sum(&self) -> Self::SumOutput {
        Self::SumOutput::from(self.parameter1) + Self::SumOutput::from(self.parameter2) + Self::SumOutput::from(self.parameter3)
    }
    // worst case scenario 0 - 255 - 255 = -510
    type SubOutput = i16;
    fn sub(&self) -> Self::SubOutput {
        Self::SubOutput::from(self.parameter1) - Self::SubOutput::from(self.parameter2) - Self::SubOutput::from(self.parameter3)
    }
    // worst case scenario 255 * 255 * 255 = 16581375
    type MulOutput = u32;
    fn mul(&self) -> Self::MulOutput {
        Self::MulOutput::from(self.parameter1) * Self::MulOutput::from(self.parameter2) * Self::MulOutput::from(self.parameter3)
    }
    // we want float division instead of integer division
    type DivOutput = f64;
    fn div(&self) -> Self::DivOutput {
        Self::DivOutput::from(self.parameter1) / Self::DivOutput::from(self.parameter2) / Self::DivOutput::from(self.parameter3)
    }
    // remainder doesn't have limit problems
    type RemOutput = u8;
    fn rem(&self) -> Self::RemOutput {
        Self::RemOutput::from(self.parameter1) % Self::RemOutput::from(self.parameter2) % Self::RemOutput::from(self.parameter3)
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
