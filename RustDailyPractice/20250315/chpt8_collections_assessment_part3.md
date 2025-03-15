# Assessment

## Part 3: Hash Maps (20 minutes)

### Exercise 3.1: Word Frequency Counter
Implement a function that counts the frequency of each word in a text:

```rust
use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, u32> {
    // Your code here
}

// Example:
// word_frequency("hello world hello") -> {"hello": 2, "world": 1}
```

### Exercise 3.2: Merging Hash Maps
Create a function that merges two hash maps, summing the values for duplicate keys:

```rust
fn merge_maps(map1: HashMap<String, i32>, map2: HashMap<String, i32>) -> HashMap<String, i32> {
    // Your code here
}

// Example:
// map1: {"apple": 1, "orange": 2}
// map2: {"apple": 3, "banana": 4}
// Result: {"apple": 4, "orange": 2, "banana": 4}
```

### Exercise 3.3: Team Scores Tracker
Implement a function that updates a team's score. If the team doesn't exist, add it with the given score. If it does exist, only update if the new score is higher than the existing one:

```rust
fn update_team_score(scores: &mut HashMap<String, i32>, team: &str, points: i32) {
    // Your code here
}

// Example usage:
// Let's say scores = {"Blue": 10, "Yellow": 50}
// update_team_score(&mut scores, "Blue", 15) -> scores becomes {"Blue": 15, "Yellow": 50}
// update_team_score(&mut scores, "Blue", 5) -> scores remains {"Blue": 15, "Yellow": 50} (no change as 5 < 15)
// update_team_score(&mut scores, "Red", 20) -> scores becomes {"Blue": 15, "Yellow": 50, "Red": 20}
```

These exercises directly map to our learning objectives by testing your ability to:
- Create and populate hash maps
- Manage the ownership of keys and values
- Use the `.entry()` API
- Update hash maps based on existing values
- Iterate over hash map contents

Let me know when you've completed these exercises!


# my answers

```rust
use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    map
}


fn merge_maps(map1: HashMap<String, i32>, map2: HashMap<String, i32>) -> HashMap<String, i32> {
    let mut new_map = map1;

    // my approach
    // for (k, v) in map2 {
    //     if let Some(existing_value) = new_map.get_mut(&k) {
    //         *existing_value += v;
    //     } else {
    //         new_map.insert(k, v);
    //     }
    // }

    // Claude's approach, one line, more idiomatic
    for (k, v) in map2 {
        *new_map.entry(k).or_insert(0) += v;
    }
    new_map
}


fn update_team_score(scores: &mut HashMap<String, i32>, team: &str, points: i32) {
    match scores.get_mut(team) {
        Some(existing_score) => {
            if points > *existing_score {
                *existing_score = points;
            }
        },
        None => {
            scores.insert(team.to_string(), points);
        }
    }
}


fn main() {
    let map = word_frequency("hello goodbye hello hello hello goodbye");
    println!("{:?}", map);

    let map1 = HashMap::from([
        ("apple".to_string(), 1),
        ("orange".to_string(), 2),
    ]);

    let map2 = HashMap::from([
        ("apple".to_string(), 3),
        ("banana".to_string(), 4),
    ]);

    let merged = merge_maps(map1, map2);
    println!("{:?}", merged);

    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);

    println!("Initial scores: {:?}", scores);

    update_team_score(&mut scores, "Blue", 15);
    println!("After update_team_score(\"Blue\", 15): {:?}", scores);

    update_team_score(&mut scores, "Blue", 5);
    println!("After update_team_score(\"Blue\", 5): {:?}", scores);

    update_team_score(&mut scores, "Red", 20);
    println!("After update_team_score(\"Red\", 20): {:?}", scores);
}


```
