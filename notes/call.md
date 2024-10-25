```

calculate = bottom, top -> {
    bottom..=top
        : filter { e -> e % 2 == 0 }
        : sum
};

calculate = bottom, top -> {
    if top > bottom {
        sum = 0;
        for num in bottom..=top {
            if num % 2 == 0 { sum += num; };
        };
        sum
    } else {
        0
    };
};


type Foo = {
    bar | Bar,
    baz | Baz,
};

main = -> {
    foo = Foo { bar = Bar : new, baz = Baz : new };
};

```
