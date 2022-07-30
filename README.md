> Thanks for checking out Archimedes! I'm really doing this to learn more about compilers, so I'm not looking for PRs right now. I would definitely would appreciate opening issues if you run into any bugs while you're checking out the language though!

# Archimedes
Archimedes is a systems programming language that I'm making as a hobby project to learn Rust as well as how compilers work.

## To-Do:
- [ ] Get elementary parser running
- [ ] LLVM IR code-gen / custom bytecode
- [ ] Make a binary
- [ ] Start the standard library

## Why make a new language?
Why not?

## What does it look like?
```amds
#import "core/io"

decl square_root : (number: f32) -> f32
{
    let x := number;
    let y := 1.0f;
    let e := 0.000001f;

    while x - y > e {
        x = (x + y) / 2;
        y = number / x;
    }

    return x;
}

decl CONST_VAL: f32 = 2.0f;

decl main : ()
{
    let val: f32 = square_root(CONST_VAL);
    println("sqrt(2.0) = %f", val);
}
```
## Syntax Breakdown
Let me break down the syntax for you. The root of the language comes down to two keywords:

### The `decl` keyword
This indicates the definition of either what I refer to as a "complex type," or a compile-time constant (think something similar to a `#define` in C/C++ or like a `constexpr` value in C++). One important thing to note is that when using this keyword you ***must*** explicitly declare the type. Use of the walrus operator `:=` is not permitted.

### Constants
```amds
decl CONSTANT_VALUE: i32 = 0;

decl CONST_NAME: string = "some name";
```

### Complex types
I mentioned this so it's probably important that I explain what I mean by that. There are three kinds of "complex types":

#### Struct
```amds
decl Vec3 : struct {
    x: f32,
    y: f32,
    z: f32,
}
```
#### Enum
```amds
decl Key : struct { ... }
decl MouseButton : struct { ... }

decl Event : enum {
    KeyPressed(Key),
    KeyReleased(Key),
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),
    AppQuit,
}
```
#### Procedure
```amds
decl add : (a: i32, b: i32) -> i32 {
    return a + b;
}
```

### The `let` keyword
This is used to define normal local variables (it's almost exactly the same as how rust uses the `let` keyword, with one big exception we'll go over later)

### Typing
The standard formula for defining typed data is as follows:
```amds
// Explicit typing (required when using `decl`)
decl IDENTIFIER: TYPE = VALUE;

// Explicit typing
let name: type = value;

// Implicit typing
let name := value;
```

### Mutability
The only mutability rule is that procedure parameters are always immutable.

### Pointers
Pointers are allowed and I currently have no plan on adding references.

### Metaprogramming
**I'M STILL TRYING TO FIGURE OUT A GOOD WAY TO DO THIS**
