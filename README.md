# rcal

rcal is a powerful, lightweight command-line calculator written in Rust. It supports a wide range of mathematical operations, functions, variables, and precise error reporting.

## Features

- **Professional Unit System**: Full dimensional analysis using SI base units (Length, Mass, Time, etc.).
- **User-defined functions**: Define your own functions like `f(x, y) = x^2 + y^2` and reuse them.
- **Script Mode**: Execute complex calculations from `.rcal` files.
- **Comment Support**: Use `#` for documentation in scripts and interactive mode.
- **Dimensional Safety**: Prevents impossible calculations like `5m + 10s`.
- **Case-Sensitivity**: Correct handling of physical units (e.g., `Pa`, `Hz`, `N`, `J`).
- **Implicit Multiplication**: Natural syntax support like `10m / 2s` or `2pi`.
- **Arithmetic operations**: addition, subtraction, multiplication, division, modulo, and exponentiation.
- **Advanced mathematics**: factorials and a variety of trigonometric and logarithmic functions.
- **Statistics and aggregates**: calculate sum, average, minimum, and maximum of multiple values.
- **Programmer tools**: bitwise operations (AND, OR, XOR, NOT, shifts) and number format conversion.
- **CLI UX**: syntax highlighting, tab-completion for functions/constants, and command history.
- **Variables**: assign values to variables and reuse them in subsequent calculations.
- **History**: the result of the last successful calculation is automatically stored in the `ans` variable.
- **Number formats**: support for decimal, scientific notation, hexadecimal (`0x`), and binary (`0b`).
- **Degree support**: easy degree-to-radian conversion using the `deg` constant.
- **Multi-expression support**: evaluate multiple expressions in a single line using semicolons.
- **Precise error reporting**: location-aware error messages for syntax and mathematical errors.

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

The interactive shell supports:
- **Tab-completion** for built-in functions and constants.
- **Syntax highlighting** for improved readability.
- **Arrow-key history** to browse and edit previous commands.
- **`list` command** to see all defined variables and functions.

### Non-interactive Mode

Pass the expression as a command-line argument:

```bash
cargo run -- "10 + 5 * 2"
```

### Script Execution

Pass a file path to execute a script:

```bash
cargo run -- my_script.rcal
```

Scripts support multiple lines and comments using `#`. Error reports will include line numbers.

## Mathematical Reference

### Units & Dimensional Analysis

Units are case-sensitive. `rcal` performs full dimensional analysis on all calculations.

- **Length**: `m`, `cm`, `mm`, `km`, `inch`, `ft`
- **Mass**: `kg`, `g`
- **Time**: `s`, `min`, `h`
- **Pressure**: `bar`, `atm`
- **Energy**: `Wh`, `kWh`, `eV`
- **Volume**: `l` (Liter)
- **Angles**: `rad`, `deg`
- **Derived Units**: `N` (Newton), `J` (Joule), `W` (Watt), `Pa` (Pascal), `Hz` (Hertz)

### Operations

- `+` Addition
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `%` Modulo
- `^` Exponentiation
- `!` Factorial (integers only)
- `=` Assignment / Function Definition
- `in` Unit conversion (e.g., `100km/h in m/s`)
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
- `floor(x)`, `ceil(x)`: Round down or up.
- `round(val, places)`: Round to specified decimal places.
- `exp(x)`: Exponential function (e^x).
- `clamp(val, min, max)`: Restrict value to a range.

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
- `c`: Speed of light in vacuum.
- `G`: Newtonian constant of gravitation.
- `planck`: Planck constant.
- `k_b`: Boltzmann constant.
- `Na`: Avogadro constant.
- `g0`: Standard acceleration of gravity.
- `deg`: Constant to convert degrees to radians (e.g., `sin(90 deg)`).

## Examples

Unit calculation and Dimensional Safety:
```text
> 100km / (1h)
= 27.77777777777778 m s^-1
> 5N * 2m
= 10 m^2 kg s^-2
> 5m + 10s
Math Error: Dimension mismatch
```

Unit conversion:
```text
> 100km/h in m/s
= 27.77777777777778 m s^-1
> 1kWh in J
= 3600000 J
> 3600J in Wh
= 1 Wh
```

Scientific calculations:
```text
> planck * c / 500nm in eV
= 2.4845489103322144 eV
```

Basic calculation:
```text
> 5 + 3 * 2
= 11
```

User-defined functions:
```text
> f(x) = x^2 + 10
> f(5)
= 35
> hyp(a, b) = sqrt(a^2 + b^2)
> hyp(3, 4)
= 5
```

Using variables and 'ans':
```text
> radius = 5m
> area = pi * radius^2
= 78.53981633974483 m^2
> ans / 2
= 39.269908169872415 m^2
```

Listing definitions:
```text
> x = 10; f(a) = a * 2; list
x: 10
ans: 0
f(a) = (a * 2)
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
cargo run -- "x = 10m; y = 20m; (x + y) / 2"
= 15 m
```

## Error Handling

rcal provides precise feedback when an error occurs:

```text
> 10 / (5 - 5)
10 / (5 - 5)
   ^-- Math Error: Division by zero

> 5m + 10s
5m + 10s
   ^-- Math Error: Dimension mismatch
```

When running scripts, it also includes line numbers:

```text
Error in line 7:
result = f(x, y)
            ^-- Math Error: Dimension mismatch
```

## License

This project is licensed under the GNU General Public License v3.0.
