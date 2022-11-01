# Teaching Rust to my wife

This aims to be a series of blog posts about my journey into teaching Rust to my wife.<br/>
My wife knows nothing about development, so it's the perfect test subject for a guide starting from zero.

## Lesson 2

### Structs

Structs are like logical groups of variable and associated functions. Every function associated to the struct may, or may not, access to struct instance, that's called `self` (lowercase). For example, a variable is an instance of it's type, so you can think of a function accessing to struct instance like a function that access to the variable itself, and that's called a `method`. An associated function that doesn't access to struct instance, but returns a struct instance, the same type it's associated to, shortly `Self` (it's a type, so the first letter is uppercase), is ofter called a `constructor`.

#### Example

Let's take for example a date. A date is composed by a day, that goes from 1 to 31, so an `u8` will be enough, a month, that goes from 1 to 12, so another `u8` will be enough, and an year, that can goes from... well, wikipedia says recorder history starts arouth the 4th millennium BC, so an `i16` should be enough.

```rust
struct Date {
    day: u8,
    month: u8,
    year: i16
}
```

To that struct we can associate one or more functions, like e.g. a `constructor`

```
impl Date {
    fn new(day: u8, month: u8, year: i16) -> Self {
        Date {
            day,
            month,
            year,
        }
    }
}
```

You can call an associated function like that

```rust
Date::new(1, 2, 3)
```

Let's say we need another function to know if a date is BC or AC

```
impl Date {
    fn is_bc(&self) -> bool {
        self.year < 0
    }
}
```

As you can see, this function takes a reference to self, this way it doesn't consume the instance when we use it.<br/>
We can call it like that

```rust
let a = Date::new(1, 2, 3);
a.is_bc()
```

Or even like that

```rust
let a = Date::new(1, 2, 3);
Date::is_bc(&a)
```

There is no difference, both styles are accepted.

## Homeworks

Write a struct with 2 fields of type `u8`, implement for it a constructor and then a method to sum, substract, multiply and divide the two numbers, printing the results.<br/>
Then create another struct that does the same with 3 fields.

### Traits

If a Struct is a logical groups of variable and associated functions, a trait is a behaviour, composed by associated functions and associated types.

In Rust traits define everything, every behaviour is decided by a trait implementation: formatting, addition, conversion, etc...

#### Example

Lt's say we are coding a virtual world, we have humans, dogs, birds and butterflies.

```rust
struct Human {}

struct Dog {}

struct Bird {}

struct Butterfly {}
```

Now we are implementing the walking behaviour. Basically, whoever implements the `Walk` trait is able to walk, and we only need to know with how many legs does the implementor walks.

```rust
trait Walk {
    fn legs() -> u8;
}

impl Walk for Human {
    fn legs() -> u8 {
        2
    }
}

impl Walk for Dog {
    fn legs() -> u8 {
        4
    }
}

impl Walk for Bird {
    fn legs() -> u8 {
        2
    }
}

impl Walk for Butterfly {
    fn legs() -> u8 {
        6
    }
}
```

Let's say we want to do the same with the flying behaviour, whoever implements the `Fly` trait is able to fly, and we only need to know with how many wings does the implementor flyes.

```rust
trait Fly {
    fn wings() -> u8;
}

impl Fly for Bird {
    fn Wings() -> u8 {
        2
    }
}

impl Fly for Butterfly {
    fn Wings() -> u8 {
        4
    }
}
```

As you can see, `Human` and `Dog` doesn't implement the `Fly` trait.

#### Default implementations

A trait associated functions may have a default implementation, that may be overridden in trait implementations for a type.

```rust
trait Example {
    fn example(&self) {
        println!("The default behaviour haven't been overridden");
    }
}

impl Example for u8 {}

impl Example for i8 {
    fn example(&self) {
        println!("{}", self);
    }
}
```

In this example, calling `Example::example` on an `u8` will print "The default behaviour haven't been overridden" as in default implementation, while calling it on an `i8` will print it's value.

#### Prerequisites

A trait can depend on other traits. That means that, to implement that trait, also the prerequired traits needs to be implemented. That also means that a default implementation can count on the prerequisite traits behaviours.

```rust
trait Example: std::fmt::Display {
    fn example(&self) {
        println!("{}", self);
    }
}

impl Example for u8 {}

impl Example for i8 {}
```

In this example we require on the [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) trait from Rust's standard library (`std`), that is the same trait that the `{}` formatting placeholder relies on. Since `Display` is already implemented for every number type, we can entirely rely on default implementation.

#### Associated Types

An associated type is needed where some part of a trait is variable. An associated type may have prerequisites, just like traits itself.

```rust
trait Operation {
    type Output: Prerequisite;
    fn operation(self) -> Self::Output;
}
```

For example, let's take the previous homework about structs, as we learned in our first lesson, operations on numbers can violate the boundaries of the type, e.g. substracting between `u8` can result in a number below 0. You may thinking about resorting to `i8`, the signed counterpart of `u8`, but 0 - 255 is a valid operation between `u8` and it's result won't fit inside an `i8`, so you'll need to use an `i16` as result type.

```rust
trait Subtraction {
    type Output;
    fn subtract(&self) -> Self::Output;
}

struct Example {
    a: u8,
    b: u8,
}

impl Subtraction for Example {
    type Output = i16;
    fn subtract(&self) -> Self::Output {
        Self::Output::from(self.a) - Self::Output::from(self.b)
    }
}
```

That `from` method we are using since Lesson 1 comes from a trait too, the [`From`](https://doc.rust-lang.org/std/convert/trait.From.html) trait from std.

## Homeworks

Create an `Operations` trait that has a method for every operation between numbers, then convert structs from previous homework about structs into using the new trait.

### Generics

Generics are a way to write code that works for a wide range of types.

For example, let's take the previous homework about structs, you wrote a scruct that contains 2 `u8` fields, to make it work with `i8` you would need to rewrite the entire struct, with another name, only to change the type of the fields. What if we can be generic over the type?

```rust
struct Example<T> {
    a: T,
    b: T,
}
```

We just wrote a struct that can accept every type, without restrictions. Well, maybe we have been too generic, don't you think?<br/>
For this example, we'll only consider standard number operations, without conversions.

```rust
use std::ops::{Add, Sub, Mul, Div, Rem};

struct Example<T>
where
    T: Add + Sub + Mul + Div + Rem,
{
    a: T,
    b: T,
}

impl<T> Example<T>
where
    T: Add + Sub + Mul + Div + Rem,
{
    fn add(self) -> <T as Add>::Output {
        self.a + self.b
    }
    fn sub(self) -> <T as Sub>::Output {
        self.a - self.b
    }
    fn mul(self) -> <T as Mul>::Output {
        self.a * self.b
    }
    fn div(self) -> <T as Div>::Output {
        self.a / self.b
    }
    fn rem(self) -> <T as Rem>::Output {
        self.a % self.b
    }
}
```

What are we doing here? We are requiring that the generic T of our struct implements all the 5 listed traits, this avoids building the struct with an unwanted type. Then we implement struct methods for every T with the same constraints. The output of every method is the associated type of involved trait, since all traits have an Output associate type we need to disambiguate which trait we are using every time. Again, this is because those behaviours are controlled by traits, and we can leverage them on out advantage.

We could have written the same concept another way:

```rust
use std::ops::{Add, Sub, Mul, Div, Rem};

struct Example<T> {
    a: T,
    b: T,
}

impl<T: Add> Example<T> {
    fn add(self) -> T::Output {
        self.a + self.b
    }
}

impl<T: Sub> Example<T> {
    fn sub(self) -> T::Output {
        self.a - self.b
    }
}

impl<T: Mul> Example<T> {
    fn mul(self) -> T::Output {
        self.a * self.b
    }
}

impl<T: Div> Example<T> {
    fn div(self) -> T::Output {
        self.a / self.b
    }
}

impl<T: Rem> Example<T> {
    fn rem(self) -> T::Output {
        self.a % self.b
    }
}
```

Difference is quite subtle, here we can build the struct with any T, but methods will be available only if T implements the involved trait, and we don't need to disambiguate since we are using one trait at once.<br/>
Also, you can see a more compat constraints definition, that, on the other side, can become quite messy with many constraints.

Generics can work also in functions

```rust
fn print<T>(subject: T)
where
    T: std::fmt::Display,
{
    println!("{}", subject);
}
```

The same thing can be written another way

```rust
fn print(subject: impl std::fmt::Display) {
    println!("{}", subject);
}
```

What's the difference? The `impl trait` way is surely more human readable, but is not usable when you need a slightly more complex scenario. For example, this isn't writable on that form

```rust
fn add<T: Add>(a: T, b: T) -> T::Output {
    a + b
}
```

Also functions associated to structs and traits can have generics.

## Homeworks

Combine the previous homeworks from this lesson, create 2 structs with a generic, one with 2 fields, one with 3 fields, and implement the Operations trait for them.
