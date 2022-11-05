# Teaching Rust to my wife

This aims to be a series of blog posts about my journey into teaching Rust to my wife.<br/>
My wife knows nothing about development, so it's the perfect test subject for a guide starting from zero.

## Lesson 1

### Comments

To add a comment you can use `//`, the compiler will ignore anything after that on the same line

```rust
// this is a comment
```

To write multiline comments, you can start with `/*` and end it with `*/`

```rust
/* this
is a multiline
comment */
```

### Data types

Rust is strongly typed, that means that it needs to know the exact type of everyting. Also Rust is smart enough to guess most of the types of your variables.

_What do you mean by "it's smart enough"?_<br/>
Wait a moment, we'll see later

#### Numbers

Numbers, like in all programming languages, divides in two main families:
* Integers, aka natural numbers
* Floating points, aka real numbers

Hint: You can put `_` everywhere inside a number to make it more readable.

```rust
1000000
1_000_000
1_0_0_0_0_0_0_
```

##### Integers

Integer itself divides into two families:
* Signed, numbers that can be less that zero, it's type names starts with an `i`
* Unsigned, numbers that can't be less than zero, it's type names starts with an `u`

Every Integer type is named over it's size in bits

_What's a bit?_<br/>
Computers works in binary, everything is made up of bits, a bit can only be 0 or 1 (that's wht it's called binary, only two possible values), 8bits are 1 Byte, you commonly use Bytes when talking about file sizes.

E.g. `u8` is an unsigned integer with size 8bit, that means it's value goes from 0 to 255<br/>
_Why 255?_<br/>
Well, with 8 bits you can have 2 elevated at the eighth power possible values, that means 256 values, but one of those is 0, hence 255 is the maximum value.

E.g. `i8` is a signed integer with size 8bit, that means it's value goes from -128 to 127<br/>
_Wait a moment, why `u8` goes up to 255 and `i8` only to `127`?_<br/>
That's because the sign takes away 1 bit, and only 7 bits are left for the value<br/>
_Ok, and why the minimum value is -128 and not -127?_<br/>
Well, you don't count 0 two times, don't you?

Rust comes with integers of 8bits, 16bits, 32bits and 64bits.

You can put the type at the end of the number to force it.

```rust
10
123_u8
9_i64
```

##### Floating Points

Floating point numbers are always signed and are represented using an exponent and a fraction.<br/>
Not all values are representable with this technique, and incrementing the size of a float type improves it's precision.<br/>
Rust comes with 32bit (`f32`) and 64bit (`f64`) types.

Floating point numbers are written with a `.` dividing units from decimals

```rust
0.0
2_f32
123_456.789_f64
```

#### Boolean

`bool` is a type that can only be `true` or `false`

#### Characters

`char` type represents a single character, being it a letter, number, symbol or even emoji, in any existing alphabet. It's size can vary from 1 to 4 bytes.<br/>
_What does that mean "from 1 to 4 bytes"?_<br/>
Well, we need to open a little parenthesis here. Once upon a time, almost every nation had it's own character encoding. Americans had the famous ASCII, single byte because 256 characters were enough for them, nations using latin alphabets has variations of ASCII, remaining single byte, including, for example, accented letters. Nations using different alphabet, for example Japan, had completely different characters encoding. Communicating between areas with different character encodings was difficult, you had to use conversion tables, that was an error prone operation. Luckily things have evolved since then, and now we have a common character encoding called UTF-8. AS you can imagine, to contain every single possible character from so many alphabets, 256 characters aren't enough. UTF-8 is ASCII compatible, so ASCII characters are single byte.

_If 256 characters aren't enough, how many are there?_<br/>
Stating [Wikipedia](https://en.wikipedia.org/wiki/UTF-8) there are 1112064 characters in UTF-8

Chars are represented surrounded by single quotes, e.g.:
```rust
'a'
```

#### Array

Arrays are sized list of values of the same type. Arrays can't be expanded or shrinked, if you need a list of values that can grow in size you're looking for a Vector, but we'll talk about it in another lesson.

```rust
[1, 2, 3, 4]// this has type [u8; 4]
```

_Can I have a real life example?_<br/>
A box of crayons can be seen as an array, they are all of the same type, a crayon, but different values/colors. The box is sized, like a 24 crayons box, with the difference that Rust won't allow empty space in the box if you lose a crayon.

#### Tuples

Tuples are set of values of any type. Like arrays, tuples can't be expanded or shrinked.

```rust
(1, true, 'a')// this has type (u8, bool, char)
```

_Can I have a real life example?_<br/>
The case you had at school is like a tuple, inside there are elements of different types: a pen, an eraser, a ruler, etc...

### Basic numeric operations

Basic numeric operations are sum, subtraction, multiplication, division and modulo.

Keep in mind that, on a standard situation, all operations can be performed only on values of the same type and will return another value of the same type.

_It's the famous "you can't add oranges and apples"?_<br/>
Well, kind of, but later we'll see how to turn oranges into apples.

#### Sum

```rust
1 + 2
```

#### Subtraction

```rust
1 - 2
```

#### Multiplication

```rust
1 * 2
```

#### Division

```rust
1 / 2
```

#### Remainder

```rust
1 % 2
```

### Comparison operations

Comparison operations can only be `true` or `false`, hence it returns a `bool` value.

#### Equals

```rust
1 == 2
```

#### Minor than

```rust
1 < 2
```

#### Major than

```rust
1 > 2
```

#### Minor or equal

```rust
1 <= 2
```

#### Major or equal

```rust
1 >= 2
```

### Logical operations

Logical operations can be applied between two `bool` values, and returns a `bool` value itself.

#### And

Written `&&`, returns `true` only when both values are `true`.

```rust
let a = 1:
let b = 2;
a == 1 && b == 2
```

#### Or

Written `||`, returns `true` when at least one values is `true`.

```rust
let a = 1:
let b = 2;
a == 2 || b == 2
```

### Declaring a variable

To declare a new variable you need to use the `let` keyword.

```rust
let a = 1;
let b = a + 2;
```

Here we haven't declared the type of our variables, the compiler only know they are some kind on integer and will try it's best to make it work with the rest of the code. If unsure, he'll output an error, pointing this line of code, asking to disambiguate the type.

If you want to declare also the type of your variable, the syntax is

```rust
let a: u8 = 1;
```

#### Shadowing

Shadowing is the act of declaring another variable with the same name of an existing variable. The old variable will still exists, but won't be reachable while the new one is alive.

```rust
let a = 1;
let a = 2;
```

_Isn't it futile?_<br/>
It could seem, but combined with other things it becomes quite handy, we'll see in other lessons.

_Can I undo the shadowing?_<br/>
Yes, but we'll see it later .

#### Type conversions

There are two main ways to convert between types

##### Unchecked conversions

```rust
let a: u8 = 255;
let b = a as i8;
```

Remember that i8 maximum value is 127? So what will the value of `b` be?<br/>
Well, the `as` operator will keep the binary value while changing the type. `255_u8` binary value is `11111111`, that in an i8 means `-1`.<br/>
You just found out why unchecked conversions aren't a best pratice.

##### Checked conversions

```rust
let a: u8 = 255;
let b = i8::from(a);// this is not possible, you'll get an compile error, because u8 can't be safely converted to i8
let b = i16::from(a);// this works because i16 can manage u8 max value
let b = i8::try_from(a);// this works because it's a fallible operation
```

For now, take that syntax as it is, we'll see in next lesson what does that means.

### Assign numeric operations

Assign numeric operations are used to edit a variable basing the operation on it's current value.

#### Sum

```rust
a += 1;
```

#### Subtraction

```rust
a -= 1;
```

#### Multiplication

```rust
a *= 2;
```

#### Division

```rust
a /= 2;
```

#### Remainder

```rust
a %= 2;
```

### Functions

Functions are building blocks of the language, you can see a function like a block of reusable code. The signature of a function is a contract both on the outside and on the inside of the function. That means that who's calling the function can trust the signature and the body of the function must respect the signature, not the other way around.

A function is marked with `fn`, has a name, has any number of parameters, every parameter with it's type, returns a type, if omitted returns `()` (void), and is followed by a code block.

```rust
fn function_without_parameters_that_returns_void() {
    println!("Hello");
}

fn function_with_a_signle_u8_parameter_that_explicitly_returns_u8(parameter: u8) -> u8 {
    println!("parameter value is {parameter}");
    return parameter;
}

fn function_with_two_parameters_that_implicitly_returns_u8(parameter1: u8, parameter2: u8) -> u8 {
    parameter1 - parameter2
}
```

Every instruction must end with `;`, the last instruction can omit the `;`, that's implicitly the return value.

In an executable, the `main` function is the entrypoint.

_What's an entrypoint?_<br/>
That's the code executed when your application starts, and when it returns the application exits.

#### Code blocks

The body of a function is a code block, but code blocks are everywhere you put code between braces.

```rust
let a = if b > 0 {
    1
} else {
    2
};
```

As you can see, also the two bodies of an `if/else` are code blocks, and they returns the value from last operation if not ending by `;`.

```rust
fn always_returns_two(b: u8) -> u8 {
    let a = if b > 0 {
        1
    } else {
        return 2;
    };
    a + 1
}
```

This function, like the name suggests, always returns 2, because the `return` operation values for the function and not for the code block.

```rust
let a = 1;
{
    let a = 2;
}
println!("{a}");//this will print 1
```

Here the shadowing automatically ends at the end of the code block.

### Conditions

Boolean conditions can be checked with the `if` operator, followed by a block, and optionally by an `else` and another block

```rust
if a > 1 {
    // ...
}

if a < 10 {
    // ...
} else {
    // ...
}
```

### Printing

To output a string from your code you can use the `println!` macro.

```rust
println!("Hello world");
```

To print a value, you can use the `{}` placeholder

```rust
println!("The value of a is {}", a);
```

`println!` can capture variables from the environment, e.g. those two lines of code does the same

```rust
println!("{}", a);
println!("{a}");
```

We'll talk extensively about macros in another lesson, for now keep in mind macros aren't magical, they'll expand to code you could write yourself, but they hide complexity and help reduce code repetition.

#### Debug Printing

Sometimes we need to print values that aren't printable, and we need them quick and dirty. To do that we can use the `{:?}` placeholder

```rust
println!("{:?}", a);
println!("{a:?}");
```

We''ll talk extensively about formatting in another lesson.

### Ownership

Ownership is a base concept of Rust. Think about a real life situation, if I give you a pen, that pen now is in your full control, you can do whatever you want with it.<br/>
Now, to that real life situation, let's add a Rust rule: everything that goes out of scope is automatically destroyed.<br/>
That means that, when I give my pen to you, if you don't give it back to me, or to someone else, when you finished using it, that's lost forever.<br/>
May sound a bit harsh, but you're avoiding hiring a person to clean up a pile of pens at the end of the day.

#### Borrowing

How to avoid destroying all those pens? Well, there are real life situations like that, think about postal offices or banks, where public accessible pens are chained to the desk, this way you can't take them with you, when you finish using them, they return to their owner.<br/>
In Rust those are called references.

```rust
let a = 1;
do_something(&a);// we are passing a reference to a to the function that requires a reference to u8


fn do_something(b: &u8) {
    // ...
}
```

Since in Rust everything that goes out of scope is automatically destroyed, the compiler itself denies situations where you try to keep a reference to an element after it's death (in other language this is called use-after-free, and it's a big problem).

##### Slices

A reference to an array is called a slice, and loses the size from it's type

```rust
let a: [u8; 4] = [1, 2, 3, 4];
do_something(&a);// we are passing a reference to a to the function that requires a slice of u8

fn do_something(a: &[u8]) {
    // ...
}
```

#### Mutability

In Rust everything is read-only by default, to make it editable you need to declare it mutable.

That is valid for variables

```rust
let mut a = 0;
let b = 1;
a = b + 1;
```

That is also valid for owned parameters

```rust
fn do_something(mut a: u8) {
    a = 0;
    // ...
}
```

That is also valid for reference parameters

```rust
fn do_something(a: &mut u8) {
    *a = 0;// to edit the referenced value we use the dereferencing operator *
    // ...
}
```

### Strings

In Rust, under the hood, strings are similar to array of bytes that can be read also as arrays of characters.<br/>
Remember `char` size can vary from 1 to 4 bytes? That means that a string of 10 bytes can contain 10 or less `char`.<br/>
Like arrays, if you need a string that can grow in size you're looking for a `String`.<br/>
Unlike arrays, the base type `str` isn't usable alone, you'll always see that as a reference `&str`.
We'll talk extensively about that in another lesson.

### Homeworks

Write a function for every possible operation between two u8. E.g.:

```rust
fn sum(parameter1: u8, parameter2: u8) -> u8 {
    parameter1 + parameter2
}
```

Keep in mind that certain operations can violate the boundaries of the type, manage it as needed.
