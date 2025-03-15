# Questions

You're absolutely correct! C doesn't have the `**` operator for powers either - you would use the `pow()` function from the `math.h` library instead.

Now, let's proceed with the assessment. We'll go through each collection type systematically so you can apply what you've learned.

# Rust Collections Assessment

## Part 1: Vectors (15 minutes)

### Exercise 1.1: Vector Creation and Manipulation
Implement a function that creates a vector of integers from 1 to 10, doubles each value, and returns the modified vector:

```rust
fn double_vector() -> Vec<i32> {
    // Your code here
}
```

### Exercise 1.2: Vector Element Access
Implement a function that safely retrieves the element at a given index and returns it wrapped in an Option:

```rust
fn safe_get(vec: &Vec<i32>, index: usize) -> Option<i32> {
    // Your code here
}
```

### Exercise 1.3: Mixed Types Vector
Create a function that constructs and returns a vector containing different types of spreadsheet cell data:

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn create_spreadsheet_row() -> Vec<SpreadsheetCell> {
    // Create a vector with at least one of each variant type
    // Your code here
}
```

Let me know when you've completed this section and want to proceed to Part 2.


# Answers

```rust
fn double_vector() -> Vec<i32> {

    // my implementation:
    // let mut vector = Vec::new();
    // for i in 1..=10 {
    //     vector.push(i);
    // }

    // vector.iter().map(|x| x * 2).collect()

    // Claude's approach:
    (1..=10).map(|x| x * 2).collect()
}

fn safe_get(vec: &Vec<i32>, index: usize) -> Option<i32> {

    // my implementation:
    // let element: Option<&i32> = vec.get(index);
    // match element {
    //     Some(element) => Some(*element),
    //     None => None,
    // }

    // Claude's approach:
    // vec.get(index).map(|&x| x)

    // Claude's another approach:
    vec.get(index).copied()

}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn create_spreadsheet_row() -> Vec<SpreadsheetCell> {
    let row = vec![
        SpreadsheetCell::Int(47),
        SpreadsheetCell::Float(2.236),
        SpreadsheetCell::Text(String::from("existence preceeds essence.")),
    ];
    row
}

fn main() {
    let v1 = double_vector();
    println!("doubled: {:?}", v1);

    let v2 = double_vector();
    let elmt = safe_get(&v2, 3);
    println!("element: {:?}", elmt);

    let row = create_spreadsheet_row();
    println!("Speadsheet row: {:?}", row);
}


```
