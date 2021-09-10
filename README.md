# fortytwo-lang

fortytwo-lang (FTL) is a programming language. The syntax is a mix of C and Python.
It is based on the programming language _Kaleidoscope_ from an
[LLVM tutorial](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/index.html).

The goal for FTL is to compile to LLVM IR at some point in the future.

See [Future Thoughts](docs/future_thoughts.md) for how the language could look like later on.

## Installation

### Install binary

1. [Install rust](https://www.rust-lang.org/tools/install)
2. 
```
cargo install --git https://github.com/linuskmr/fortytwo-lang
```

### Docker (soon)

1. [Install docker](https://docs.docker.com/get-docker/)

### Compile from source

```
git clone https://github.com/linuskmr/fortytwo-lang
cd fortytwo-lang
cargo build --release
```

## Documentation

1. [Install rust](https://www.rust-lang.org/tools/install)
2.
```
cargo doc --document-private-items --open
```

## Reserved keywords

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
