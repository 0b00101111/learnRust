// Create a simple user database that handles missing users properly.
#[derive(Clone)]
struct User {
    id: u32,
    username: String,
    email: String,
}

// This simulates a database of users
fn get_user_by_id(id: u32) -> Option<User> {
    let users = vec![
        User { id: 1, username: String::from("alice"), email: String::from("alice@example.com") },
        User { id: 2, username: String::from("bob"), email: String::from("bob@example.com") },
        User { id: 3, username: String::from("charlie"), email: String::from("charlie@example.com") },
    ];

    // TODO: Return Some(user) if the user with the given id exists
    // Otherwise, return None

    // for user in users {
    //     if user.id == id {
    //         return Some(user);
    //     }
    // }
    // None

    // alternative: using iter
    // iter() gives reference, so it returns a Some(&User), not a user.
    // .cloned() is used to return an owned(not reference) use.
    // to use .cloned(), the Copy trait needs to be implemented to the User struct.
    users.iter().find(|u| u.id == id).cloned()
}

// TODO: Implement the send_email function
// It should take an Option<User> and:
// - If Some(user): Return a string saying "Email sent to {username} at {email}"
// - If None: Return a string saying "No user found"
fn send_email(user: Option<User>) -> String {
    if let Some(user) = user {
        format!("Email sent to {} at {}", user.username, user.email)
    } else {
        String::from("No user found")
    }
}

fn main() {
    let user2 = get_user_by_id(2);
    let user4 = get_user_by_id(4);

    println!("{}", send_email(user2));
    println!("{}", send_email(user4));
}
