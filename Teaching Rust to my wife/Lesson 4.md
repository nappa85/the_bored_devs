# Teaching Rust to my wife

This aims to be a series of blog posts about my journey into teaching Rust to my wife.<br/>
My wife knows nothing about development, so it's the perfect test subject for a guide starting from zero.

## Lesson 3

### Matches

`match` is a really powerful operation

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
    One(u8),// this variant has a type associated, the type can be any type, even a struct or another enum
    Multiple { parameter1: u8, parameter2: u8 },// this variant is costructed like a struct herself
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

Now, we know how generics works, and they works also on enums, so we can make that enum generic to any type, no?

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Well, we just reinvented [std::option::Option](https://doc.rust-lang.org/std/option/enum.Option.html).<br/>
More, there is no need to do `use std::option::Option` in our projects, becase it's automatically done for us.<br/>
You don't even need to specify `Option::Some` or `Option::None` every time, you yust need to write `Some` or `None`.

#### Results

What if we can apply the same concept to failing operations?<br/>
You may think you just need a `bool`, `true` on successful operation, `false` on error.<br/>
Ok, what if you want to return something on successful operation?<br/>
You may thin in other languages you can do that returning `null` on error, so translated to Rust it would be an `Option` with `Some` on success and `None` on error.<br/>
Ok, but what if we want to return something also on error?<br/>
Let's say something that describes the error, maybe there are several error types and we need to tell to the user exactly which error happened.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Sounds quite exhaustive, right?<br/>
We have an `Ok` variant with an associated type, and an `Err` variant with a different associated type.<br/>
Well, we just reinvented [std::result::Result](https://doc.rust-lang.org/std/result/enum.Result.html).<br/>
Like `Option`, there is no need to do `use std::result::Result`, and you can directly write `Ok` and `Err`.<br/>
Every fallible operation in Rust returns a `Result`, without reinventing the wheel every time.
