enum Weather {
    Sunny(f32), //Temperature in Celsius
    Cloudy(f32),
    Rainy(f32),
    Snowy(f32),
}

// TODO: Implement the get_activity function
// It should recommend activities based on weather conditions and temperature
fn get_activity(weather: Weather) -> String {
    // Use match to handle all weather conditions
    // For Sunny: If temperature > 30, suggest "Stay hydrated and find shade"
    //            Otherwise, suggest "Perfect for a picnic!"
    // For Cloudy: If temperature < 15, suggest "Bring a jacket"
    //            Otherwise, suggest "Good day for a walk"
    // For Rainy: If temperature < 10, suggest "Stay inside with a book"
    //            Otherwise, suggest "Bring an umbrella"
    // For Snowy: Always suggest "Build a snowman!"
    match weather {
        Weather::Sunny(temperature) => {
            if temperature > 30.0 {
                String::from("Stay hydrated and find shade")
            } else {
                String::from("Perfect for a picnic!")
            }
        },
        Weather::Cloudy(temperature) => {
            if temperature < 15.0 {
                String::from("Bring a jacket")
            } else {
                String::from("Good day for a walk")
            }
        },
        Weather::Rainy(temperature) => {
            if temperature < 10.0 {
                String::from("Stay inside with a book")
            } else {
                String::from("Bring an umbrella")
            }
        },
        Weather::Snowy(_temperature) => {
            String::from("Build a snowman!")
        }
    }
}


fn main() {
    let hot_day = Weather::Sunny(35.0);
    let cold_rain = Weather::Rainy(5.0);
    let cloudy_day = Weather::Cloudy(18.0);
    let snow_day = Weather::Snowy(-2.0);

    println!("When it's hot and sunny: {}", get_activity(hot_day));
    println!("When it's cold and rainy: {}", get_activity(cold_rain));
    println!("When it's cloudy: {}", get_activity(cloudy_day));
    println!("When it's snow: {}", get_activity(snow_day));
}

