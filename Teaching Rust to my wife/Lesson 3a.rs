
//! Print the numbers from 0 to 9 with every possible cycle.

fn main() {
    let mut a = 0;
    loop {
        println!("{a}");
        a += 1;
        if a >= 10 {
            break;
        }
    }

    let mut a = 0;
    while a < 10 {
        println!("{a}");
        a += 1;
    }

    for a in 0..10 {
        println!("{a}");
    }
}
