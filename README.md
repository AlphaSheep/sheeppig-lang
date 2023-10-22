# SheepPig Lang

A very simple statically typed functional language. This was written as an exercise in writing an interpreter. It is not intended to be used for anything serious.

![mascot](./assets/sheeppig.png)


## Getting started

See the `samples` folder for some example programs.

Run `cargo test` to run the test suite.

Run `cargo run` to run the current program in `main.rs`.

## Language Features

The language is a very simple statically typed functional language. It has the following types:

* Integers
* Floats
* Booleans
* Characters
* Strings
* None
* Functions

An example hello world program:
```
fun main() {
    println("Hello, World!")
}
```

And this is a somewhat more complex program:
```
using {
    sqrt, sin from math
    read_file as read from fs
}

/*
  This is a block comment
  Useful for documenting functions
*/
fun main() {
    # This is an inline comment  
    var a: int = 1  # Declaring a mutable variable
    pi: float = 3.141592   # Declaring an immutable variable

    a = a + 2  # Variable reassignment
    a += 3  # Increment a variable

    b = do_stuff(a)
    println(b)
}

fun do_stuff(value: int): string {
    # New lines end a statement
    b: string = "Hello"
    c: char = 'C'

    # Escaping the newline allows contiuing on the next line
    long_variable = -1 + 2 - (3 * 4 / 5) % 6 ** 7 << 8 >> 9 + \
        10 & 11 | 12 ^ 13 + ~14
    
    if a > 1 && 2 < 3 || 4 >= 5 && !(6 <= 7) || 8 == 9 && 10 != 11 {
        return "this was fun"
    } else {
        return "or was it?"
    }
}
```
