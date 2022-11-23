
//! Write a function that receives a `&str` and returns the count of non-ASCII characters.

fn count_non_ascii(s: &str) -> usize {
    s.chars().filter(|c| !c.is_ascii()).count()
}

fn main() {
    let a = "Lorem ipsum dolor sit amet";
    println!("The string \"{a}\" contains {} non-ASCII characters", count_non_ascii(a));

    let a = "❤ß山İ٣7৬¾①ℝ💣K藏三δAΔ中💝";
    println!("The string \"{a}\" contains {} non-ASCII characters", count_non_ascii(a));
}
