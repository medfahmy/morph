#[macro_export]
macro_rules! map {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = std::collections::HashMap::new();
            $(
                assert!($x.len() == 2, "Map elements should be of length 2");
                temp_vec.insert($x[0], $x[1]);
            )*
            temp_vec
        }
    };
} 

// macro map {
// (x,)* => {
//
// }
// }
