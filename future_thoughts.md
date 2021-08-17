# Future Thoughts

This is how the language could look like later on...

## Hello World

Print "Hello World" to stdout.

```python
print("Hello World")
```

## Variables

Variables are declared with the `var` keyword follwed by a type seperated by a `:`.

You can assign a value to the variable directly by specifying an `=` after the variable
and then the value itself.

```python
var a: int = 42
```

Variables are mutable:

```python
var a: int = 42
a = 1337
```

If the type is inferable by the compiler, you can omit the type:


```python
var a = 42
# a is of type int
```

Without assignment, the content of the variable is zeroed.

```python
var a: int
# a is implicit zeroed,
# so printing it is ok
print(a)
```

> Note: `var` works like a normal variable as you would expect
and doesn't do the confusing things that happen in JavaScript with `var`.

### Binary Expressions

Binary expressions work just like you would expect:
Brackets before multiplication/division before addition/substraction

```python
var b = 2 * (3 + 1)
# b == 8
```

### Naming Conventions for Variables

In general, the naming conventions follow those of Python:

- Normal variables should start with a lowercase letter and be in sneak_case, like `my_variable_with_long_name`.

- Variables/Constants that should not be reassigned/modified are in UPPER_CASE and
words are seperated with `_`, like `MY_CONSTANT_WITH_LONG_NAME`.

- Variables starting with `_` (one underscore) are considered private.

- Variables starting with `__` (double underscore) are builtin or language internal,
like `__FTL_VERSION`, which is also marked as constant.

> Note: These are just naming conventions. FTL does not enforce private or constant variables.

---

## Data Types

### Basic Data Types

int, int8, int16, int32, int64, uint8, uint16, uint32, uint64, float32, float64, bool

nothing, any (only for pointers)

### Strings

Strings (`str`) are denoted with `"` at the beginning and at the end.
For strings, the plus operator is overloaded so that strings can be concatenated.
Other data types can also be concatenated with `+`, as long as a constructor of `str()` is implemented for them.
For details see the chapter about [structs, constructors and associated functions](#structs).

```python
var a: str = "Hello"
a = a + " World "
a = a + 42
# a == "Hello World 42"
```

#### String Interpolation / Format Strings

```python
var language = "FTL"
var number = 42
print("{language} is awesome, because {number} is the answer to everything.")
# Prints:
# "FTL is awesome"
```

### Arrays

```python
var my_int_arr: arr<int>(10)
my_int_arr@3 = 42
# my_int_arr@3 == 42
```

## Pointers

Load data from a pointer:

```python
# Pointer to an int
var my_int_ptr: ptr int = 0x4628
var my_int = deref myintptr

# C void pointer
var my_ptr: ptr any = 0x51941
var my_int_ptr = my_ptr as ptr int
var my_int = deref my_int_ptr
```

Get the address of a variable:

```python
var a = 42
var a_ptr: ptr int = ref a
```

---

## Functions

Functions are defined with the `def` keyword. Parameters have a name followed by a `:` and a type. The return type is specified after the parameters. If the function does not return anything, you may omit the return value or explicit specify `nothing` as return type (For details see [Data Types](#data-types)).

```typescript
def add(x: int, y: int): int {
    return x + y
}

var result = add(3, 5)
assert(result, 8)
```

### Calling libc Functions

With the `extern` keyword, you can import and use a functions from the C standard library in your FTL programm.

```typescript
# Import function
extern write(fd: int, msg: ptr char, strlen: uint64);

var text = "hallo"

write(1, text._cstr, text._len)
```

### Naming Conventions for Functions

In general, the naming conventions follow those of Python:

- Normal functions should start with a lowercase letter and be in sneak_case, like `my_function_with_long_name`.

- Variables starting with `_` (one underscore) are considered private.

- Variables starting with `__` (double underscore) are builtin or language internal,
like `__logic_and()`.

> Note: These are just naming conventions. FTL does not check calling private functions.

---

### Structs

Define a `struct`. The declaration looks similar to that of a function.

```typescript
struct Person(
    name: str,
    age: uint8
)
```

Allocate a new `Person` by defining a variable with explicit type. This performs an allocation and zeros all bytes. 

```typescript
var p: Person
p.name = "Linus"
```

Or allocate a new `Person` with the `alloc` keyword.

```typescript
var linus = alloc Person
linus.age = 19
```

```typescript
var linus = new Person(name="Linus", age=19)
```

### Associated Functions

FTL makes associated function pretty simple.
Any function `f` that takes an object of type `T` as its first parameter can also be
directly called on an object `t` of type `T` with `t.f()`.

Example:

The function `say_hello()` takes a person and prints a greeting.
You can call this function directly on a concrete person with `person.say_hello()`.

```python
def say_hello(p: Person) {
   print("{p.name} says hello")
}

linus.say_hello()
# Is syntatic sugar for
say_hello(linus)
```

### Constructors

It's a convention to create a constructor for a struct by creating a function named exactly as the struct,
which returns an object of this struct. This constructor may take any number of arguments.

```python
struct Person (
    name: str,
    age: uint8
)

# Constructor for Person
def Person(name: str, age: uint8): Person {
    return Person(name, age)
}

var linus = Person("Linus", 19)
```

#### Casting

Casting works simply by calling the constructor of the type you want to get.

```python
var i: int = 42
var f: float = float(42)
```

<details>
    <summary>Implementation of int to float casting</summary>

    ```python
    def float(x: int) {
        return __int_to_float(x)
    }
    ```
    
    > Note that `__int_to_float()` is a builtin function provided by FTL.
</details>

If you want to be able to cast your own type `A` to another type `B`, you have to implement a constructor
for for `B` taking `A` as parameter. Then you can cast an instance `a` of `A` to an instance of `B` by calling
`b = B(a)`.

For example, if you want to cast a Person to a int (for whatever reason one would want to do that),
you can implement this like so:

```python
def int(person: Person): int {
    return person.age
}
```

If you create a type `T`, you may want to implement `bool(T)` to able to cast it to a bool
and `str(T)` to be able to print it.

### Operator Overloading

You can overload operators for you custom types. You simply define a function with a special name (see below) and you can use
the operator for your type 🥳.

Example:

```python
def __equals(me: Person, other: Person): bool {
   return me.name == other.name and me.age == other.age
}

var linus1: Person
linus1.name = "Linus"
linus2.age = 19

var linus2: Person
linus2.name = "Linus"
linus2.age = 19

if linus1 == linus2 {
   print("linus1 and linus2 are equal")
} else {
   print("linus1 and linus2 are not equal")
}
```

| Operator | Function Name   | Parameters        | Return Value |
| -------- | --------------- | ----------------- | ------------ |
| +        | __plus          | self: A, other: B | A            |
| -        | __minus         | self: A, other: B | A            |
| /        | __div           | self: A, other: B | A            |
| *        | __times         | self: A, other: B | A            |
| <        | __less          | self: A, other: B | bool         |
| <=       | __less_equal    | self: A, other: B | bool         |
| >        | __greater       | self: A, other: B | bool         |
| >=       | __greater_equal | self: A, other: B | bool         |
| ==       | __equal         | self: A, other: B | bool         |
| =/=      | __not_equal     | self: A, other: B | bool         |
| bitor    | __bitor         | self: A, other: B | A            |
| mod      | __mod           | self: A, other: B | A            |
| bitor    | __bitor         | self: A, other: B | A            |
| bitand   | __bitand        | self: A, other: B | A            |


### Naming Conventions for Structs

In general, the naming conventions follow those of Python classes:

- Normal structs should start with a uppercase letter and be in CamelCase, like `MyStructWithLongName`.

- Structs starting with `_` (one underscore) are considered private.

- Structs starting with `__` (double underscore) are builtin or language internal.

> Note: These are just naming conventions. FTL does not check calling private functions.


## Error

TODO...

```python
error "An error message"
# Prints:
# Error in main.ftl line 14
# An error message
```

## Generics

TODO...

```python
def plus<T>(first: T, second: T): T {
   return first + second
}

plus(42, 1337)
plus(4.2, 12.37)
```
