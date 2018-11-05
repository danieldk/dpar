macro_rules! ok_or {
    ($expr:expr, $none_expr:expr) => {
        match $expr {
            Some(val) => val,
            None => $none_expr,
        }
    };
}
