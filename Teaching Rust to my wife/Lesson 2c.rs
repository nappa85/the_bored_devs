
//! Combine the previous homeworks from this lesson, create 2 structs with a generic, one with 2 fields, one with 3 fields, and implement the Operations trait for them.

use std::ops::{Add, Sub, Mul, Div, Rem};

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

// here we create a trait to control type conversion in operations between two operands
trait OperationTwoType: Copy + Sized {
    type SumOutput: From<Self> + Add<Output=Self::SumOutput>;
    type SubOutput: From<Self> + Sub<Output=Self::SubOutput>;
    type MulOutput: From<Self> + Mul<Output=Self::MulOutput>;
    type DivOutput: From<Self> + Div<Output=Self::DivOutput>;
    type RemOutput: From<Self> + Rem<Output=Self::RemOutput>;
}

// here we create a trait to control type conversion in operations between three operands
trait OperationThreeType: Copy + Sized {
    type SumOutput: From<Self> + Add<Output=Self::SumOutput>;
    type SubOutput: From<Self> + Sub<Output=Self::SubOutput>;
    type MulOutput: From<Self> + Mul<Output=Self::MulOutput>;
    type DivOutput: From<Self> + Div<Output=Self::DivOutput>;
    type RemOutput: From<Self> + Rem<Output=Self::RemOutput>;
}

struct OperationsTwo<T> {
    parameter1: T,
    parameter2: T,
}

impl<T> OperationsTwo<T> {
    fn new(parameter1: T, parameter2: T) -> Self {
        OperationsTwo { parameter1, parameter2 }
    }
}

impl<T: OperationTwoType> Operations for OperationsTwo<T> {
    type SumOutput = T::SumOutput;
    fn sum(&self) -> Self::SumOutput {
        Self::SumOutput::from(self.parameter1) + Self::SumOutput::from(self.parameter2)
    }
    type SubOutput = T::SubOutput;
    fn sub(&self) -> Self::SubOutput {
        Self::SubOutput::from(self.parameter1) - Self::SubOutput::from(self.parameter2)
    }
    type MulOutput = T::MulOutput;
    fn mul(&self) -> Self::MulOutput {
        Self::MulOutput::from(self.parameter1) * Self::MulOutput::from(self.parameter2)
    }
    type DivOutput = T::DivOutput;
    fn div(&self) -> Self::DivOutput {
        Self::DivOutput::from(self.parameter1) / Self::DivOutput::from(self.parameter2)
    }
    type RemOutput = T::RemOutput;
    fn rem(&self) -> Self::RemOutput {
        Self::RemOutput::from(self.parameter1) % Self::RemOutput::from(self.parameter2)
    }
}

struct OperationsThree<T> {
    parameter1: T,
    parameter2: T,
    parameter3: T,
}

impl<T> OperationsThree<T> {
    fn new(parameter1: T, parameter2: T, parameter3: T) -> Self {
        OperationsThree { parameter1, parameter2, parameter3 }
    }
}

impl<T: OperationThreeType> Operations for OperationsThree<T> {
    type SumOutput = T::SumOutput;
    fn sum(&self) -> Self::SumOutput {
        Self::SumOutput::from(self.parameter1) + Self::SumOutput::from(self.parameter2) + Self::SumOutput::from(self.parameter3)
    }
    type SubOutput = T::SubOutput;
    fn sub(&self) -> Self::SubOutput {
        Self::SubOutput::from(self.parameter1) - Self::SubOutput::from(self.parameter2) - Self::SubOutput::from(self.parameter3)
    }
    type MulOutput = T::MulOutput;
    fn mul(&self) -> Self::MulOutput {
        Self::MulOutput::from(self.parameter1) * Self::MulOutput::from(self.parameter2) * Self::MulOutput::from(self.parameter3)
    }
    type DivOutput = T::DivOutput;
    fn div(&self) -> Self::DivOutput {
        Self::DivOutput::from(self.parameter1) / Self::DivOutput::from(self.parameter2) / Self::DivOutput::from(self.parameter3)
    }
    type RemOutput = T::RemOutput;
    fn rem(&self) -> Self::RemOutput {
        Self::RemOutput::from(self.parameter1) % Self::RemOutput::from(self.parameter2) % Self::RemOutput::from(self.parameter3)
    }
}

// at this point everything is generic and we only need to implement OperationTwoType and OperationThreeType for the desided types
impl OperationTwoType for u8 {
    // worst case scenario 255 + 255 = 510
    type SumOutput = u16;
    // worst case scenario 0 - 255 = -255
    type SubOutput = i16;
    // worst case scenario 255 * 255 = 65025
    type MulOutput = u16;
    // we want float division instead of integer division
    type DivOutput = f64;
    // remainder doesn't have limit problems
    type RemOutput = u8;
}

impl OperationThreeType for u8 {
    // worst case scenario 255 + 255 + 255 = 765
    type SumOutput = u16;
    // worst case scenario 0 - 255 - 255 = -510
    type SubOutput = i16;
    // worst case scenario 255 * 255 * 255 = 16581375
    type MulOutput = u32;
    // we want float division instead of integer division
    type DivOutput = f64;
    // remainder doesn't have limit problems
    type RemOutput = u8;
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
