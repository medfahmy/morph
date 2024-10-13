```
type Option<T> =
| Some(T)
| None

type Point<T> = {
    x: T,
    y: T,
}

impl Point<T: Add> {
    distance | Self, Self -> Float
    distance = (self, other) -> {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        (dx * dx + dy * dy).sqrt()
    }
}

distance(p1, p2) {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    (dx * dx + dy * dy).sqrt()
}

main = () -> {
    let mut count = 0;
    
    let points = [Point{x: 0, y: 0}, Point{x: 3, y: 4}];
    
    let (sender, receiver) = channel();
    
    spawn {
        for p in points {
            sender.send(p);
        }
        sender.close();
    }
    
    while let Some(point) = receiver.receive() {
        count += 1;
        println("Received point: ({}, {})", point.x, point.y);
    }
    
    println("Processed {} points", count);
    
    let d = distance(points[0], points[1]);
    println("Distance: {}", d);
}

main = () -> {
    let x = 5;
    let mut y = 5;
    y += 1;
}

modify = (mut x) {
    x += 1;
    x
}

add = (a, b) -> {
    a + b
}

apply = (f, a) -> {
    f(a)
}

increment = (a) -> {
    a + 1
}

main = () -> {
    print(apply(increment, a));
}

identity = (x) -> {
    x
}

maybe_add = (x, y) -> {
    match x {
        Some(val) => val + y,
        None => y,
    }
}

apply_twice = (f, x) -> {
    f(f(x))
}

read_file = (path) -> {
    fs::read_to_string(path)
}

divide = (a, b) -> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

calculate = () -> {
    let res1 = divide(10, 2)?;
    let res2 = divide(20, res1)?;
    Ok(res2)
}

check_value = (x) -> {
    if x > 0 {
        "Positive"
    } else {
        "Non-positive"
    }
}

process_result = (res) -> {
    match res {
        Ok(val) => val,
        Err(_) => 0,
    }
}

producer = (channel) -> {
    for i in 0..10 {
        channel.send(i);   // Type inference: channel is inferred to send integers (i32)
    }
}

consumer = (channel) -> {
    while let Some(data) = channel.receive() {   // Type inference: channel receives integers (i32)
        println!("Received: {}", data);
    }
}

main = () -> {
    let channel = Channel::new();  // Channel type is inferred
    
    spawn(producer(channel.clone()));  // Start the producer process
    spawn(consumer(channel));          // Start the consumer process
}

mutable_worker = (mut state, channel) -> {
    while let Some(data) = channel.receive() {
        state.push(data);   // Mutating state within the worker process
    }
}

select_example = (channel1, channel2) -> {
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

main = () -> {
    let channel = Channel::with_capacity(5);  // Buffer size is explicit, but message type is inferred
    
    spawn(producer(channel.clone()));  // Type inference continues here
    spawn(consumer(channel));
}

struct Counter {
    count: Int,
}
```
