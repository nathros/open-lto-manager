/* Concatenate several static &str into single one at compile time */
#[macro_export]
macro_rules! static_concat {
    ($($x: expr),* $(,)?) => { const {
        const LEN: usize = 0 $(+ $x.len())*;
        let ret = &const {
            let mut ret = [0u8; LEN];
            let mut ret_idx = 0;
            $(
            let x: &::core::primitive::str = $x; // Catch any weird mistakes with a let-binding
            let mut x_idx = 0;
            while x_idx < x.len() {
                ret[ret_idx] = x.as_bytes()[x_idx];
                x_idx += 1;
                ret_idx += 1;
            }
            )*
            ret
        };
        match ::core::str::from_utf8(ret) {
            Ok(x) => x,
            Err(_) => panic!(),
        }
    }}
}
