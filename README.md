# fortytwo-lang

fortytwo-lang (FTL) is a programming language. The syntax is a mix of C and Python.
It is based on the programming language _Kaleidoscope_ from an
[LLVM tutorial](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/index.html).

The goal for FTL is to compile to LLVM IR at some point in the future.

See [Future Thoughts](docs/future_thoughts.md) for how the language could look like later on.

## [Installation](docs/installation.md)

## Sourcecode Documentation

To get the documentation of the fortytwo-lang sourcecode, run the following [on your local computer](docs/installation.md#compile-yourself):

```
cargo doc --document-private-items --open
```

## Reserved keywords

You may don't use these for variable names as they could get keywords in the future.

**Memory:**
ref
deref
alloc
del
new
default
nil

**Math:**
shl
shr
bitxor
bitor
bitand

**Logic:**
bool
true
false
and
or
xor
not

**Data structures:**
struct
arr
const
char
string
list
enum

**Loops:**
for
in
of
while

**Useful stuff:**
debug
print
error
def
extern

**Integer data types:**
int8
uint8
int16
uint16
int32
uint32
int64
uint64

**Floating point number data types:**
float32
float64
