# rcal

rcal is a powerful, lightweight command-line calculator written in Rust. It supports a wide range of mathematical operations, functions, variables, and precise error reporting.

## Features

- Arithmetic operations: addition, subtraction, multiplication, division, modulo, and exponentiation.
- Advanced mathematics: factorials and a variety of trigonometric and logarithmic functions.
- Statistics and aggregates: calculate sum, average, minimum, and maximum of multiple values.
- Programmer tools: bitwise operations (AND, OR, XOR, NOT, shifts) and number format conversion.
- Variables: assign values to variables and reuse them in subsequent calculations.
- History: the result of the last successful calculation is automatically stored in the 'ans' variable.
- Number formats: support for decimal, scientific notation, hexadecimal (0x), and binary (0b).
- Degree support: easy degree-to-radian conversion using the 'deg' constant.
- Multi-expression support: evaluate multiple expressions in a single line using semicolons.
- Precise error reporting: location-aware error messages for syntax and mathematical errors.
- Overflow protection: comprehensive checks to prevent silent calculation errors.

## Installation

Ensure you have Rust and Cargo installed on your system. Clone the repository and build the project:

```bash
cargo build --release
```

The executable will be available at `target/release/rcal`.

## Usage

rcal can be used in both interactive and non-interactive modes.

### Interactive Mode

Simply run the executable without arguments to start the interactive shell:

```bash
cargo run
```

### Non-interactive Mode

Pass the expression as a command-line argument:

```bash
cargo run -- "10 + 5 * 2"
```

## Mathematical Reference

### Operations

- `+` Addition
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `%` Modulo
- `^` Exponentiation
- `!` Factorial (integers only)
- `=` Assignment
- `,` Argument separator
- `;` Expression separator

### Functions

#### General
- `sin(x)`, `cos(x)`, `tan(x)`: Trigonometric functions (input in radians).
- `asin(x)`, `acos(x)`, `atan(x)`: Inverse trigonometric functions.
- `sqrt(x)`: Square root.
- `abs(x)`: Absolute value.
- `ln(x)`: Natural logarithm.
- `log(x)`: Logarithm base 10.

#### Aggregates
- `sum(a, b, ...)`: Sum of all arguments.
- `avg(a, b, ...)`: Average of all arguments.
- `min(a, b, ...)`: Minimum of all arguments.
- `max(a, b, ...)`: Maximum of all arguments.

#### Bitwise (Programmer)
- `and(a, b)`: Bitwise AND.
- `or(a, b)`: Bitwise OR.
- `xor(a, b)`: Bitwise XOR.
- `not(a)`: Bitwise NOT.
- `lshift(a, n)`: Left shift `a` by `n` bits.
- `rshift(a, n)`: Right shift `a` by `n` bits.
- `hex(x)`: Formats the integer result as hexadecimal.
- `bin(x)`: Formats the integer result as binary.

### Constants

- `pi`: The ratio of a circle's circumference to its diameter.
- `e`: Euler's number.
- `deg`: Constant to convert degrees to radians (e.g., `sin(90 deg)`).

## Examples

Basic calculation:
```text
> 5 + 3 * 2
= 11
```

Using variables and 'ans':
```text
> radius = 5
> area = pi * radius^2
= 78.53981633974483
> ans / 2
= 39.269908169872415
```

Using aggregates:
```text
> avg(10, 20, 30, 40)
= 25
> max(5, 12, 3)
= 12
```

Using bitwise operations and formats:
```text
> 0xff + 0b1010
= 265
> hex(and(ans, 0xf))
= 0x9
```

Using degrees:
```text
> sin(90 deg)
= 1
```

Sequential execution:
```bash
cargo run -- "x = 10; y = 20; (x + y) / 2"
= 15
```

## Error Handling

rcal provides precise feedback when an error occurs:

```text
> 10 / (5 - 5)
10 / (5 - 5)
   ^-- Math Error: Division by zero
```

## License

This project is licensed under the GNU General Public License v3.0.
