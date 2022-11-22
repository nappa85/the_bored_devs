# Teaching Rust to my wife

This aims to be a series of blog posts about my journey into teaching Rust to my wife.<br/>
My wife knows nothing about development, so it's the perfect test subject for a guide starting from zero.

## Lesson 3

### Ranges

Ranges are sets of number starting from a value and/or ending to another value.

Ranges can be bounded or unbounded, the starting value is always inclusive, the ending values can be exclusive or inclusive.

```rust
.. // full unbounded range
0.. // range bounded below
..10 // range bounded above
..=10 // range bounded inclusively above
0..10 // range bounded non inclusively
0..=10 // range bounded inclusively
```

### Cycles

#### Control Flow

Every cycle can be aborted with the `break` operator, or skipped with the `continue` operator.

#### Loop

`loop` is the symplies of cycles, it can only be controlled with control flow operators, else it's an infinite loop.

```rust
let mut a = 0;
loop {
    a += 1;
    if a > 10 {
        break;
    }
}
```

#### While

`while` is a cycle with a condition. The condition is evaluated at every turn, as soos as the condition become `false` the cycle exits.

```rust
let mut a = 0;
while a < 10 {
    a += 1;
}
```

#### For

`for` is a cycle that iterates over anything that implements the trait `IntoIterator`. Classic examples of implementors of `IntoIterator` are arrays, slices and ranges.

```rust
for a in 0..10 {

}
```

### Closures

Closures are alternative ways to define functions.

```rust
let closure_without_parameters = || {
    println!("hello");
};
let closure_with_an_untyped_parameter = |a| {
    println!("{a}");
    a
};
let closure_with_two_typed_parameters_and_return_type = |a: u8, b: u8| -> u8 {
    println!("{a} + {b} = {}", a + b);
    a + b
};
```

If a closure body is made of a single instruction, braces can be omitted.

```rust
let closure_made_of_a_single_instruction = || println!("hello");
```

Closures can capture variables from the environment.

```rust
let mut a = 1;
let print_a = || println!("{a}");// this closure will contain a reference to a
print_a();// will print 1
let b = &mut a;
let mut increment = || *b += 1;
increment();
increment();
println!("{b}");// will print 3
```

Closures are data types, so they can be passed as parameters to functions. Closures have also unnameable types, therefore you must use generics.

```rust
fn do_something_with_a_closure<F>(closure: F)
where
    F: Fn(),
{
    closure();
}

do_something_with_a_closure(|| println!("hello"));
```

### Iterators

Iterators are implementors of trait `Iterator`, they can be seen as sequences of data.

Iterators are quite handy to manipurale data.

```rust
[1, 2, 3, 4, 5, 6]
    .into_iter()
    .filter(|n| n % 2 == 0)
    .for_each(|n| println!("{n}"));
```

This code transforms the array in an iterator, filters the elements keeping only even values and then print them.

```rust
"Lorem ipsum dolor sit amet"
    .chars()
    .filter(|c| c.is_uppercase())
    .count()
```

This code iterates over chars contained in a string, filter only uppercase chars and count them.
