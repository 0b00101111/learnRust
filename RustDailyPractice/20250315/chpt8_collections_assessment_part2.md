# Assessment:

## Part 2: Strings (20 minutes)

### Exercise 2.1: String Concatenation
Write a function that takes a vector of strings and concatenates them with a separator:

```rust
fn join_strings(strings: Vec<&str>, separator: &str) -> String {
    // Your code here
}

// Example: join_strings(vec!["Hello", "world", "Rust"], ", ") -> "Hello, world, Rust"
```

### Exercise 2.2: UTF-8 Character Counting
Implement a function that counts the number of characters (not bytes) in a string:

```rust
fn count_chars(s: &str) -> usize {
    // Your code here
}

// Example:
// count_chars("Hello") -> 5
// count_chars("こんにちは") -> 5 (not 15, which would be the byte count)
```

### Exercise 2.3: String Manipulation
Create a function that reverses words in a sentence while keeping the original word order:

```rust
fn reverse_words(s: &str) -> String {
    // Example: "hello world" -> "olleh dlrow"
    // Your code here
}
```

Let me know when you've completed these exercises and are ready for feedback or want to move on to Part 3.

## Revised Part 2: Strings Assessment

### Exercise 2.1: String Type Understanding
Create both a string slice and a String, then explain the difference between them:

```rust
fn demonstrate_string_types() {
    // Create both a &str and a String
    // Then print both of them
    // Add a comment explaining the key differences
}
```

### Exercise 2.2: String Modification
Create a function that takes a string and appends additional text using the methods we learned:

```rust
fn build_greeting(name: &str) -> String {
    // Create a new String
    // Use push_str and/or push to build a greeting message
    // Return the complete greeting
    // Example output: "Hello, Jane! Welcome."
}
```

### Exercise 2.3: String Concatenation
Write a function that concatenates strings using both the + operator and format! macro:

```rust
fn combine_strings(part1: &str, part2: &str, part3: &str) -> (String, String) {
    // First combine the strings using the + operator
    // Then combine them using the format! macro
    // Return both results as a tuple
}
```

This revision properly aligns with our learning objectives and only requires techniques that were explicitly covered in the instructional phase.


# My Answer:

```rust
fn join_strings(strings: Vec<&str>, separator: &str) -> String {
    strings.join(separator)
}


fn count_chars(s: &str) ->usize {
    s.chars().count()
}

fn reverse_words(s: &str) -> String {
    // my approach which was wrong. It's each word that needs to be reversed
    // not the entire string literal.
    //s.chars().rev().collect()

    // Claude's approach
    s.split_whitespace()
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

// revised exercise towards the objectives

fn demonstrate_string_types() {
    // Create both a &str and a String
    // Then print both of them
    // Add a comment explaining the key differences

    let s1 = "hello there";  //this is a reference of a string literal, the type is &str
    let s2 = String::from("so long my friend");  // this is a String type, it's owned
    println!("s1: {}", s1);
    println!("s2: {}", s2);
}

fn build_greeting(name: &str) -> String {
    // Create a new String
    // Use push_str and/or push to build a greeting message
    // Return the complete greeting
    // Example output: "Hello, Jane! Welcome."

    let mut s1 = String::new();
    s1.push_str("Hi, ");
    s1.push_str(name);
    s1.push('.');

    s1
}

fn combine_strings(part1: &str, part2: &str, part3: &str) -> (String, String) {
    // First combine the strings using the + operator
    // Then combine them using the format! macro
    // Return both results as a tuple

    let s1 = part1.to_owned() + " " + part2 + " " + part3 + ".";
    println!("{}", s1);

    let s2 = format!("{part1} {part2} {part3}.");
    println!("{}", s2);

    (s1, s2)
}

fn main() {
    let joined = join_strings(vec!["Hello", "world", "Rust"], ", ");
    println!("{}", joined);

    let count = count_chars("Hello");
    println!("{}", count);

    let reversed = reverse_words("hello world");
    println!("{}", reversed);

    demonstrate_string_types();

    println!("{}", build_greeting("Sam"));

    combine_strings("existence", "preceeds", "essence");

}

```
