# Future Thoughts

This is how the language could look like later on...

## Hello World

```python
print "Hello World"
```

## Variables

Separate initializations and declaration:

```python
var a: int
# a is implicit zeroed,
# so printing it is ok
print a
```

Initialization and declaration combined:

```python
var a: int = 42
```

Type inference:

```python
var a = 42
# a is of type int
```

Variables are mutable:

```python
var a = 42
a = a + 1
```

## Error

```python
error "An error message"
# Prints:
# Error in line 14: error("An error message")
# "An error message"
```

## Binary expressions

```python
var b = 2 * 3 + 1
# b == 7
```

## Functions

A Function which adds 3 to its only parameter:

```python
def add3(x: int) -> int:
    return x + 3
;
# Note: `;` is used to end a block

var result = add3(5)
assert(result, 8)
```

`nothing` is a data type and its only variant is `nothing`.

## Primitive casting

```python
var i: int = 42
var f: float = i as float
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

## Strings

Null-terminated C string:

```python
var s1: ptr char = "Hallo"
var s2: cstr = "Hallo"
```

Note: `ptr char` and `cstr` are equivalent.

Growable string from the stdlib:

```python
var a: str = str_from("Hello")
a = a + " World "
a = a + 42
# a == "Hello World 42"
```

## Array access

```python
var my_int_arr: arr 10 int
my_int_arr@3 = 42
# my_int_arr@3 == 42
```

## Structs

```python
struct Person:
    name: str,
    age: uint8
;

var linus = Person{name: str::from("Linus"), age: 19}
print(linus.name + " is " + linus.age + " years old")
```

### Associated functions

```python
def say_hello(me: Person):
   print(me.name, "says hello")
;

linus.say_hello()
# Is syntatic sugar for
say_hello(linus)
```

## Operator overloading

```python
def operator == (me: Person, other: Person) -> bool:
   return me.name == other.name and me.age == other.age
;

var linus1 = Person{name: str::from("Linus"), age: 19}
var linus2 = Person{name: str::from("Linus"), age: 19}
if linus1 == linus2:
   print("linus1 and linus2 are equal")
; else:
   print("linus1 and linus2 are not equal")
;
```

## Data types

int, int8, int16, int32, int64, uint8, uint16, uint32, uint64, float32, float64, bool

nothing, any (only for pointers)

## Calling libc Functions

```python
# Import function
extern write(int fd, cstr msg, uint64 strlen);
extern strlen(cstr msg) -> uint64;

var text = "hallo"

write(1, text, strlen(text))
```

## Generics

```python
def plus$T$(T first, T second) -> T:
   return first + second
;
```
