```
Option = T -> | {
    | Some(T),
    | None,
};

Option :: {
    is_some = self -> match self {
        Some(_) => true,
        None => false,
    };

    is_none = self -> match self {
        Some(_) => false,
        None => true,
    };
};


```
