# fortytwo-lang

fortytwo-lang (FTL) is a programming language. The syntax is a mix of C and Python.
It is based on the programming language _Kaleidoscope_ from an
[LLVM tutorial](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/index.html).

The goal for FTL is to compile to LLVM IR at some point in the future.

See [Future Thoughts](docs/future_thoughts.md) for how the language could look like later on.

## Installation

### Docker

For using the docker container, you need to [install docker](https://docs.docker.com/get-docker/).

#### Build docker image yourself

```
docker build -t ftl .
```

#### Use existing image

Pull the existing image from [hub.docker.com](https://hub.docker.com):

```
docker pull linuskmr/fortytwo-lang
```

Run ftl in the container. Replace `BINARY` with the binary and arguments you want to run (TODO).

```
docker run linuskmr/fortytwo-lang BINARY
```

### Compile yourself

For compiling ftl yourself, you need to [install rust](https://www.rust-lang.org/tools/install).

#### Global installation

```
cargo install --git https://github.com/linuskmr/fortytwo-lang
```

#### Compile in local folder

Download the git repository and build:

```
git clone https://github.com/linuskmr/fortytwo-lang
cd fortytwo-lang
cargo build --release
```

## Documentation

Build and open the documentation of the ftl sourcecode:

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
