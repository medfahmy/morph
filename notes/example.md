```
main | () -> Never;
main = () -> {
    x mut = 1;
    x += 1;
    println { x };
};

increment = i mut -> {
    i += 1;
};

use std.format;

Format :: format | Self, format.Formatter -> format.Result;

Option :: T: Format ->
| Some { T }
| None 
derive Format;

Point<T: Format> ::
    x: T,
    y: T,
} derive Format;

Node<T> :: {
    value: T,
    next: Option<Node<T>>,
};

LinkedList<T> :: {
    head: Node<T>,
};

impl<T> Format for Node<T> {
    format = self, f -> {
        f.write { self }
    };
};

trait ToString = {
    to_string | Self -> String;
};


s | String;
s = "hello";

foo | bool
foo = s.contains { "hell" } && !s.is_empty;


type Foo<T> =
| Bar(T)
| Baz {
    boo: Option<T>
};

impl Point<T: Add> {
    distance | Self, Self -> Self;
    distance = self, other -> {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        { dx * dx + dy * dy }.sqrt
    };
};

distance(p1, p2) = {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    { dx * dx + dy * dy }.sqrt
};

main = () -> {
    count = 0;
    points = List { 
        Point { x: 0, y: 0 }, 
        Point { x: 3, y: 4 }, 
    };
    
    sender, receiver = channel;
  
    spawn {
        for p in points {
            sender.send { p };
        };
        sender.close;
    };
    
    while Some { point } = receiver.receive {
        count += 1;
        println { "Received point: ({};, {})", point.x, point.y };
    };
    
    println { "processed {}; points", count };
    
    d = distance { points.0, points.1 };
    println { "Distance: {};", d };
};;

add = a, b -> {
    a + b
};

apply = f, a -> {
    f(a)
};

increment = a -> {
    a + 1
};

main = -> {
    print { apply { increment, a } };
};

identity = x -> {
    x
};

maybe_add = x, y -> {
    match x {
        Some { val } => val + y,
        None => y,
    };
};

apply_twice = f, x -> {
    f { f { x } }
};

read_file = path -> {
    std.fs.read_to_string { path }
};

divide = a, b -> {
    if b == 0 {
        Err { "Division by zero" }
    } else {
        Ok { a / b }
    }
};

calculate = -> {
    let res1 = divide { 10, 2 }.try;
    let res2 = divide { 20, res1 }.try;
    Ok { res2 }
};

check_value = x -> {
    if x > 0 {
        "Positive"
    } else {
        "Non-positive"
    };
};

process_result = res -> {
    match res {
        Ok { val } => val,
        Err { _ } => 0,
    }
};

producer = channel -> {
    for i in 0..10 {
        channel.send { i };   
    };
};

consumer = channel -> {
    while Some { data } = channel.receive {
        println! { "Received: {};", data };
    };
};

main = () -> {
    channel = Channel.new();  
    
    spawn(producer(channel.clone()));  
    spawn(consumer(channel));          
};

mutable_worker = (state mut, channel) -> {
    while Some(data) = channel.receive() {
        state.push(data);
    };
};

select_example = (channel1, channel2) -> {
    loop {
        select! {
            value = channel1.receive() => {
                println!("Received an integer: {};", value);
            };,
            text = channel2.receive() => {
                println!("Received a string: {};", text);
            };,
            default => {
                println!("No data received");
                break;
            };
        };
    };
};

main = () -> {
    channel = Channel.with_capacity { 5 };  
    
    spawn { producer { channel.clone } };  
    spawn { consumer { channel } };
};

type Counter {
    count: Int,
};
```

