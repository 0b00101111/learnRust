fn main() {
    // Demonstration of each exercise

    // Convert temperature
    convert_temps(100.0, 0);
    convert_temps(100.0, 1);

    // Generate nth Fibbonacci number
    let n:i32 = 10;
    let nth_fibbo = generate_fib(n);
    println!("The {n}th fibonacci number is: {nth_fibbo}.");
    
    // print "Twelv Days of Christmas"
    print_twelve_days();
}

fn convert_temps(degree: f32, format: i32) {
    if format == 0 {
        println!("Converting from Celsius to Fahrenheit...");
        let converted_degree = degree * 9.0 / 5.0 + 32.0;
        println!("{degree}\u{00B0}C is {:.2}\u{00B0}F.", converted_degree);
    } else if format == 1 {
        println!("Converting from Fahrenheit to Celsius...");
        let converted_degree = (degree - 32.0) * 5.0 / 9.0;
        println!("{degree}\u{00B0}F is {:.2}\u{00B0}C.", converted_degree);
    }
}

fn generate_fib_recursion(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        generate_fib(n - 2) + generate_fib(n - 1)
    }
}

fn generate_fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

fn print_twelve_days() {
    todo!();
}

