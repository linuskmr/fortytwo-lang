# Future Thoughts

This is how the language could look like later on...

## Hello World

```python
print("Hello World")
```

## Variables

Separate initializations and declaration:

```python
var a: int
# a is implicit zeroed,
# so printing it is ok
print(a)
```

Initialization and declaration combined:

```python
var a: int = 42
```

Type inference:

```python
var a = 42
print(__type(a))
# Prints:
# "int"
```

Variables are mutable:

```python
var a = 42
a = a + 1
```

### Binary Expressions

```python
var b = 2 * 3 + 1
# b == 7
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
var my_int_arr: arr<T>(10)
my_int_arr@3 = 42
# my_int_arr@3 == 42
```

### Structs

Define a `struct`. The declaration looks similar to that of a function.

```python
struct Person(
    name: str,
    age: uint8
)
```

Allocate a new `Person` by defining a variable with explicit type. This performs an allocation and zeros all bytes. 

```python
var p: Person
p.name = "Linus"
```

Or allocate a new `Person` with the `new` keyword.

```python
var linus = new Person
linus.age = 19
```

### Naming Conventions for Structs

In general, the naming conventions follow those of Python classes:

- Normal functions should start with a lowercase letter and be in sneak_case, like `my_function_with_long_name`.

- Variables starting with `_` (one underscore) are considered private.

- Variables starting with `__` (double underscore) are builtin or language internal,
like `__logic_and()`.

> Note: These are just naming conventions. FTL does not check calling private functions.

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

## Error

```python
error "An error message"
# Prints:
# Error in main.ftl line 14
# An error message
```

## Functions

Functions are defined with the `def` keyword. Parameters have a name followed by a `:` and a type. The return type is specified after the parameters. If the function does not return anything, you may omit the return value or explicit specify `nothing` as return type (For details see [Data Types](#data-types)).

```python
def add(x: int, y: int): int {
    return x + y
}

var result = add(3, 5)
assert(result, 8)
```

## Casting

Casting works simply by calling the constructor of the type you want to get.

```python
var i: int = 42
var f: float = float(42)
```

This is implemented like so:

```python
def float(x: int) {
    return __int_to_float(x)
}
```

Note that `__int_to_float()` is a builtin function provided by FTL.

### Associated Functions

```python
def say_hello(p: Person) {
   print("{p.name} says hello")
}

linus.say_hello()
# Is syntatic sugar for
say_hello(linus)
```

### Operator Overloading

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

## Calling libc Functions

```python
# Import function
extern write(fd: int, msg: ptr char, strlen: uint64);

var text = "hallo"

write(1, text._cstr, text._len)
```

## Generics

```python
def plus<T>(first: T, second: T): T {
   return first + second
}

plus(42, 1337)
plus(4.2, 12.37)
```
