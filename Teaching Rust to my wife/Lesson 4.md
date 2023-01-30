# Teaching Rust to my wife

This aims to be a series of blog posts about my journey into teaching Rust to my wife.<br/>
My wife knows nothing about development, so it's the perfect test subject for a guide starting from zero.

## Lesson 3

### Enums

Rust's Enums are really powerful and really pervasive in the language.<br/>
Enums are basically a list of `variants`.

```rust
enum Boolean {
    Yes,
    No,
}
```

In this example the enum `Boolean` can only value `Boolean::Yes` or `Boolean::No`.

_Isn't that a bool type?_<br/>
Well, kind of. think about asking a user input, the user must only reply "yes" or "no", but it can be any other multiple choice question.<br/>
It doesn't makes sense keeping the string around after you validated it, once you converted it in an enum variant you can discard the original input and use a much more ergonomic system.

```rust
impl From<String> for Boolean {
    fn from(s: String) -> Self {
        if s.eq_ignore_ascii_case("yes") {
            Boolean::Yes
        } else if s.eq_ignore_ascii_case("no") {
            Boolean::No
        } else {
            panic!("Unrecognized Boolean value {s}")
        }
    }
}
```

This is an example of rudimental conversion method, it's quite effective but it `panic` on error.<br/>
When an application panics, it closes itself, it's a last resort solution and it isn't a best pratice, we'll se how to manage errors in a few minutes.

Enum variants can also have associated data.

```rust
enum MultipleChoice {
    None,
    One(u8),// this variant has a type associated, the type can be any type, even a tuple, a struct or another enum
    Multiple { parameter1: u8, parameter2: u8 },// this variant is costructed like a struct herself, with named fields
}
```

Talking about memory size, an enum always weights like the biggest of it's variants, plus 1 byte if it has less than 256 variants, 2 bytes if it has less than 65536 values, etc...

#### Options

In Rust there isn't the concept of null/nil that is present in almost all other languages (and causes so many bugs).<br/>
So, how can you express the absence of something?

```rust
enum OptionalU8 {
    Some(u8),
    None,
}
```

This enum explicitly describes the possibility of absence of the value, and it must be explicitly checked, therefore there can't be a bug because you forgot to check if the values was null.

Now, we know how generics works, and they works also on enums, so we can make that enum generic to any type, right?

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Well, we just reinvented [std::option::Option](https://doc.rust-lang.org/std/option/enum.Option.html).<br/>
More, there is no need to add `use std::option::Option` to our projects, becase it's automatically done for us.<br/>
You don't even need to specify `Option::Some` or `Option::None` every time, you yust need to write `Some` or `None`.

#### Results

What if we can apply the same concept to failing operations?<br/>
You may think you just need a `bool`, `true` on successful operation, `false` on error.<br/>
Ok, what if you want to return something on successful operation?<br/>
You may think in other languages you can do that returning `null` on error, so translated to Rust it would be an `Option` with `Some` on success and `None` on error.<br/>
Ok, but what if we want to return something also on error?<br/>
Let's say something that describes the error, maybe there are several error types and we need to tell to the user exactly which error happened.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Sounds quite exhaustive, right?<br/>
We have an `Ok` variant with a generic associated type, and an `Err` variant with a different generic associated type.<br/>
Well, we just reinvented [std::result::Result](https://doc.rust-lang.org/std/result/enum.Result.html).<br/>
Like `Option`, there is no need to do `use std::result::Result`, and you can directly write `Ok` and `Err`.<br/>
Every fallible operation in Rust returns a `Result`, without reinventing the wheel every time.

Taking back previous example with Boolean enum, we can use [std::convert::TryFrom](https://doc.rust-lang.org/beta/std/convert/trait.TryFrom.html) trait to obtain a conversion method that returns the starting value on error

```rust
impl TryFrom<String> for Boolean {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.eq_ignore_ascii_case("yes") {
            Ok(Boolean::Yes)
        } else if s.eq_ignore_ascii_case("no") {
            Ok(Boolean::No)
        } else {
            Err(s)
        }
    }
}
```

### Matches

`match` is a really powerful operator.<br/>
A match is made of a subject and several patterns pointing to a code block.<br/>
Each pattern can have an optional condition.<br/>
A pattern can be a variable itself, even a `_` that means the variable is ignored.<br/>
Patterns are evaluates in the order they are written.<br/>
A match must always cover all possible values.
```rust
match subject {
    pattern if condition => {},
    patter => {},
    variable @ pattern if condition => {},
    variable @ pattern => {},
    variable => {},
}
```

You can, for example, match an integer against ranges
```rust
let a = 1;
match a {
    0 => println!("zero!"),
    x if x < 0 => println!("negative"),
    x @ 0..=9 if x % 2 == 0 => println!("even"),
    0..=9 => println!("under 10"),
    _ => println!("other"),
}
```

You can destructure structs
```rust
struct Foo {
    a: u8,
    b: u8,
}

fn do_something(foo: Foo) {
    match foo {
        Foo { a: 1, b: 2 } => println!("got 1 and 2"),
        Foo { a, .. } if a > 10 => println!("got a over 10"),// here we are ignoring b
        _ => println!("uninteresting"),
    }
}
```

You can destructure enums
```rust
enum MultipleChoice {
    None,
    One(u8),
    Multiple { parameter1: u8, parameter2: u8 },
}

fn do_something(foo: MultipleChoice) {
    match foo {
        MultipleChoice::One(1) => println!("got 1"),
        MultipleChoice::Multiple { parameter1, .. } if parameter1 > 10 => println!("got parameter1 over 10"),
        _ => println!("uninteresting"),
    }
}
```

### Try Operator

The try operator `?` is really helpful to simplify your code without losing readability.<br/>
It works on everything that implements the [std::ops::Try](https://doc.rust-lang.org/std/ops/trait.Try.html) trait, automatically returning the a "bad" kind of value and unwrapping the "good" kind.<br/>
Unfortunately the `Try` trait is still considered unstable, so you can't implement for your types, you can use it only on types for `std` that already implement it, for example `Result` and `Option`.<br/>
Even with this restriction, the try operator is really helpful

E.g. without try operator you would write something like that
```rust
fn do_something(a: u64) -> Result<u64, MyError> {
    let b = match first_check(a) {
        Ok(b) => b,
        Err(e) => return Err(e.into()),
    };
    let c = match second_check(b) {
        Ok(c) => c,
        Err(e) => return Err(e.into()),
    };
    Ok(c)
}
```

Using the try operator you can write the same like that
```rust
fn do_something(a: u64) -> Result<u64, MyError> {
    let b = first_check(a)?;
    let c = second_check(b)?;
    Ok(c)
}
```
