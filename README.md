#  rustats

A beginner-friendly statistics library written in Rust — built for learning, open to contributions.

`rustats` is a growing collection of statistics functions implemented in Rust. The goal is simple: **learn Rust by implementing real, useful functions** and get comfortable contributing to open-source projects along the way.

Whether you're writing your first Rust function or your first pull request, this is the place to start.

---

##  What's Inside

| Module | Function | Description |
|--------|----------|-------------|
| `primary` | `mean` | Arithmetic mean of a sequence of `f64` values |

> More functions are waiting to be implemented — that's where **you** come in!

---

##  Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

### Fork then Clone the Repository

```bash
git clone https://github.com/YOUR-USERNAME/rustats.git
cd rustats
```

### Run the Playground

There's a `main.rs` file you can use as a sandbox to call functions, experiment, and see results:

```bash
cargo run
```

### Run the Tests

```bash
cargo test
```

---

##  Project Structure

```
rustats/
├── src/
│   ├── lib.rs              # Crate root — re-exports all modules
│   ├── main.rs             # Playground binary for experimentation
│   └── primary/            # Domain module: primary/basic statistics
│       └── mod.rs          # Module declaration & re-exports
├── tests/                  # Integration tests
├── note.txt                # Contribution guidelines for function design
├── Cargo.toml
└── README.md
```

---

## 🤝 How to Contribute

This project is designed for beginners! Here's how you can add a new statistics function:

### 1. Pick a Function

Some ideas to get you started:

- **Primary statistics:** `median`, `mode`, `range`, `variance`, `standard_deviation`
- **Or suggest your own** — open an issue to discuss it!

### 2. Guidelines

- Functions should return None for empty input.
- Follow the standard signature pattern (fn foo(arr: &[f64]) -> Option<f64>) 
- Place exactly one function in each source file.
- Organize functions under the appropriate domain module directory.
- Domain modules should re-export their functions.
- The crate root should re-export public functions for easy access.
- Public APIs should be usable directly from the crate root

### 3. Implement It

**a)** Create a new file in the appropriate module directory:

```
src/primary/your_function.rs
```

**b)** Write your function with a doc comment and example:

```rust
/// A brief description of what the function does.
///
/// Empty input returns `None`.
///
/// # Example
/// ```
/// use rustats::primary::your_function;
///
/// let values = [1.0, 2.0, 3.0];
/// let result = your_function(values);
/// assert_eq!(result, Some(/* expected */));
/// ```
pub fn your_function(values: &[f64]) -> Option<f64>
{
    // your implementation here
}
```

**c)** Register it in `src/primary/mod.rs`:

```rust
pub mod your_function;
pub use your_function::your_function;
```

### 4. Write Tests

Add an test file in the `tests/` directory:

```
tests/your_function.rs
```

Look at [`tests/mean.rs`](tests/mean.rs) for a good example — cover edge cases like empty input, single values, negative numbers, and decimals.

### 5. Open a Pull Request

- Fork the repo
- Create a branch (`git checkout -b BRANCH-NAME`)
- Commit your changes
- Push and open a PR

