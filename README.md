# input-rust 🦀 

A simple Rust crate that mimics Python's `input()` function with error handling. It prompts the user for input, trims it, and ensures it's not empty before returning the result.

## Features
- Prompts the user for input with a custom message.
- Returns an error if the input is empty.
- Provides error handling for input failures.

## Installation

Add this to your `Cargo.toml` file:

```
[dependencies]
input-rust = "1.0.0"
```

## Usage

You can use the `input` function to prompt the user for input:

```rust
use input_rust::input;

fn main() {
    match input("Please enter some text: ") {
        Ok(value) => println!("You entered: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let text = input("Please enter some text: ").unwrap(); 
    println!("{}", text);
}
```

## Error Handling

If the user enters an empty string or an error occurs, the `input` function will return an error with a descriptive message:

- If the input is empty, an `InvalidInput` error is returned.
- If there is any other error reading the input, it returns an error of type `Other`.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
