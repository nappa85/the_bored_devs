# Teaching Rust to my wife

This aims to be a series of blog posts about my journey into teaching Rust to my wife.<br/>
My wife knows nothing about development, so it's the perfect test subject for a guide starting from zero.

## Lesson 2

### Structs

Structs are like logical groups of variable and associated functions. Every function associated to the struct may, or may not, access to struct instance, that's called `self` (lowercase). For example, a variable is an instance of it's type, so you can think of a function accessing to struct instance like a function that access to the variable itself, and that's called a `method`. An associated function that doesn't access to struct instance, but returns a struct instance, the same type it's associated to, shortly `Self` (it's a type, so the first letter is uppercase), is ofter called a `constructor`.

_Can you explain it simplier?_<br/>
When an associated function needs to access struct's data, it's a method and will have `self` as first parameter.<br/>
Else, it's an associated function. If the associated function returns an instance of the struct itself, it's a `constructor`.

#### Example

Let's take for example a date. A date is composed by a day, that goes from 1 to 31, so an `u8` will be enough, a month, that goes from 1 to 12, so another `u8` will be enough, and an year, that can goes from... well, wikipedia says recorder history starts around the 4th millennium BC, so an `i16` should be enough.

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

#### Homeworks

Write a struct with 2 fields of type `u8`, implement for it a constructor and then a method to sum, substract, multiply and divide the two numbers, printing the results.<br/>
Then create another struct that does the same with 3 fields.

### Traits

If a Struct is a logical groups of variables and associated functions, a trait is a behaviour, composed by associated functions and associated types.

In Rust every behaviour is defined by a trait implementation: formatting, addition, conversion, etc...

#### Example

Let's say we are coding a virtual world, we have humans, dogs, birds and butterflies.

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

Let's say we want to do the same with the flying behaviour, whoever implements the `Fly` trait is able to fly, and we only need to know with how many wings does the implementor flies.

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

Trait associated functions may have a default implementation, that may be overridden in trait implementations for a type.

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

In this example we require the [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) trait from Rust's standard library (`std`), that is the same trait that the `{}` formatting placeholder relies on. Since `Display` is already implemented for every number type, we can entirely rely on default implementation.

#### Associated Types

An associated type is needed when some parts of a trait are variable. An associated type may have prerequisites, just like traits itself.

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

#### Clone and Copy traits

`Clone` is a trait that controls the ability to duplicate a value. Cloning means recursively copy everything, probably at this point it won't ring a bell, but for now it's enough for you to know that there are types that can't be simply copied. `Clone` covers those types.

`Copy` is an empty trait, also called `marker`, that can be applied only to types that can bel safely copied without recursion.

For example, every base number type is `Copy`. A struct composed only by `Copy` types can be `Copy` itself. If a type is `Copy`, it's instance can survive consumption because it will be copied as needed.

Copy and Clone can be implemented with `derive macros`

```rust
#[derive(Clone, Copy)]
struct Example {
    a: u8,
    b: u8,
}
```

Also read only references are `Copy`.

#### Homeworks

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

_What are we doing here?_<br/>
We are requiring that the generic T of our struct implements all the 5 listed traits, this avoids building the struct with an unwanted type. Then we implement struct methods for every T with the same constraints. The output of every method is the associated type of involved trait, since all traits have an Output associated type we need to disambiguate which trait we are using every time. Again, this is because those behaviours are controlled by traits, and we can leverage them on our advantage.

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

_What's the difference?_<br/>
Difference is quite subtle, here we can build the struct with any T, but methods will be available only if T implements the involved trait, and we don't need to disambiguate since we are using one trait at once.<br/>
Also, you can see a more compat constraints definition, that, on the other side, can become quite messy with many constraints.

Generics can work also on functions

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

What's the difference? The `impl trait` way is surely more human readable, but it's not usable when you need a slightly more complex scenario. For example, this isn't writable on that form

```rust
fn add<T: Add>(a: T, b: T) -> T::Output {
    a + b
}
```

Also functions associated to structs and traits can have generics.

#### Generic implementation

Generics can be used also to implement a trait for anything that respects a constraint. Let's take for example this code

```rust
#[derive(Debug)]
struct Example1<T> {
    a: T,
}

#[derive(Debug)]
struct Example2<T> {
    a: T,
    b: T,
}

#[derive(Debug)]
struct Example3<T> {
    a: T,
    b: T,
    c: T,
}

trait MyTrait: std::fmt::Debug {
    fn example(&self) {
        println!("debug value is {:?}", self);
    }
}

impl<T: std::fmt::Debug> MyTrait for T {}
```

In this code we are declaring 3 different structs, automatically deriving `Debug` trait for them using a derive macro, and a trait MyTrait that requires the `Debug` trait, then, with a single operation, we are implementig MyTrait for everything implements `Debug`, not only our structs, but really everything.

Obviously this has limitations, to avoid messing up. To be able to implement a trait on something, at least one between the trait and the target must have been created in the current project.

#### Drop

Drop happens when a variable exits it's scope, and is destroyed. Standard drop simply frees the memory, but there are specials cases when other operations are needed. In those cases the [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) trait can be implemented.

```rust
struct Example {
    alarm: bool,
}

impl Example {
    fn new() -> Self {
        Example {
            alarm: true,
        }
    }
    fn dismiss(mut self) {
        self.alarm = false;
    }
}

impl Drop for Example {
    fn drop(&mut self) {
        if self.alarm {
            println!("The alarm is still active");
        }
    }
}

fn main() {
    let a = Example::new();
}
```

The code in this example will print "The alarm is still active", because variable a is dropped when exits the scope and dismiss haven't been called.

[`drop`](https://doc.rust-lang.org/std/mem/fn.drop.html) is also the name of a function that ensures the variable is destroyed at will instead of when exiting the scope. This function is really peculiar, since it's an empty function that has a generic `T` without constraints. It leverages Rust's ownership system, simply taking ownership of anything and making it immediately exit the scope. Obviously calling `drop` on something that is `Copy` won't have effects, since, if needed, you'll be dropping a copy of the value.

```rust
let a = 1;
let a = 2;
println!("{a}");//this will print 2
drop(a);
println!("{a}");//this will print 1
```

This is another example of shadowing, similar to the one with code blocks, but here we control the life of the second variable by explicitly calling `drop`.

#### Homeworks

Combine the previous homeworks from this lesson, create 2 structs with a generic, one with 2 fields, one with 3 fields, and implement the Operations trait for them.
