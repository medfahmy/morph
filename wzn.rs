// Define a struct
struct Point {
    x: float,
    y: float
}

// Function with type inference
fn distance(p1, p2) {
    let dx = p2.x - p1.x
    let dy = p2.y - p1.y
    (dx * dx + dy * dy).sqrt()
}

// Main function
fn main() {
    // Mutable variable
    let mut count = 0
    
    // Immutable variable with type inference
    let points = [Point{x: 0, y: 0}, Point{x: 3, y: 4}]
    
    // CSP-style channel for concurrency
    let (sender, receiver) = channel()
    
    // Spawn a concurrent process
    spawn {
        for p in points {
            sender.send(p)
        }
        sender.close()
    }
    
    // Receive and process points
    while let Some(point) = receiver.receive() {
        count += 1
        println("Received point: ({}, {})", point.x, point.y)
    }
    
    println("Processed {} points", count)
    
    // Using the distance function with type inference
    let d = distance(points[0], points[1])
    println("Distance: {}", d)
}

fn main() {
    let x = 5;
    let mut y = 5;
    y += 1;
}

fn increment(mut x) {
    x += 1;
    x

    let x = 10;
    modify(x); // Error: x is not mutable
}

fn modify(mut x) {
    x += 1;
    x
}

fn add(a, b) {
    a + b
}

fn apply(f, a) {
    f(a)
}

fn increment(a) {
    a + 1
}

fn main() {
    print(apply(increment, a));
}

fn identity(x) {
    x
}

fn maybe_add(x, y) {
    match x {
        Some(val) => val + y,
        None => y,
    }
}

fn apply_twice(f, x) {
    f(f(x))
}

fn read_file(path) {
    fs::read_to_string(path)
}

fn divide(a, b) {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

fn calculate() {
    let res1 = divide(10, 2)?;
    let res2 = divide(20, res1)?;
    Ok(res2)
}

fn check_value(x) {
    if x > 0 {
        "Positive"
    } else {
        "Non-positive"
    }
}

fn process_result(res) {
    match res {
        Ok(val) => val,
        Err(_) => 0,
    }
}

fn producer(channel) {
    for i in 0..10 {
        channel.send(i);   // Type inference: channel is inferred to send integers (i32)
    }
}

fn consumer(channel) {
    while let Some(data) = channel.receive() {   // Type inference: channel receives integers (i32)
        println!("Received: {}", data);
    }
}

fn main() {
    let channel = Channel::new();  // Channel type is inferred
    
    spawn(producer(channel.clone()));  // Start the producer process
    spawn(consumer(channel));          // Start the consumer process
}

fn mutable_worker(mut state, channel) {
    while let Some(data) = channel.receive() {
        state.push(data);   // Mutating state within the worker process
    }
}

fn select_example(channel1, channel2) {
    loop {
        select! {
            value = channel1.receive() => {
                println!("Received an integer: {}", value);
            },
            text = channel2.receive() => {
                println!("Received a string: {}", text);
            },
            default => {
                println!("No data received");
                break;
            }
        }
    }
}

fn main() {
    let channel = Channel::with_capacity(5);  // Buffer size is explicit, but message type is inferred
    
    spawn(producer(channel.clone()));  // Type inference continues here
    spawn(consumer(channel));
}

struct Counter {
    count: Int,
}

enum 
