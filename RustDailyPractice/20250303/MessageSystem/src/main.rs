// Exercise 1: Message System
// Implement a messaging system for a chat application with different message types:
//
// Text messages containing just a string
// Image messages containing an image URL and optional caption
// Location messages containing latitude and longitude
// Contact messages containing a name and phone number
//
// Create an enum to represent these message types and implement a function that displays the message appropriately.

enum MessageType {
    TextMessage {
        message: String,
    },
    ImageMessage {
        url: String,
    },
    LocationMessage {
        latitude: f32,
        longitude: f32,
    },
    ContactMessage {
        name: String,
        phone_number: String,
    },
}

fn display_message(message: MessageType) {
    match message {
        MessageType::TextMessage { message } => {
            println!("{}", message);
        },
        MessageType::ImageMessage { url } => {
            println!("Image link: {}", url);
        },
        MessageType::LocationMessage { latitude, longitude } => {
            println!("Location latitude: {}, longitude: {}", latitude, longitude);
        },
        MessageType::ContactMessage { name, phone_number } => {
            println!("{}'s phone_number: {}", name, phone_number);
        },
    }
}

fn main() {
    let message1 = MessageType::TextMessage {
        message: String::from("first handshake")
    };
    let message2 = MessageType::ImageMessage {
        url: String::from("https://i.imgur.com/JPKHDK2.png"),
    };
    let message3 = MessageType::LocationMessage {
        latitude: 40.5350,
        longitude: 118.7280,
    };
    let message4 = MessageType::ContactMessage {
        name: String::from("Sam"),
        phone_number: String::from("18987920324"),
    };
    display_message(message1);
    display_message(message2);
    display_message(message3);
    display_message(message4);
}