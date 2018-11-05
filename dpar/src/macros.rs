macro_rules! ok_or {
    ($expr:expr, $none_expr:expr) => {
        match $expr {
            Some(val) => val,
            None => $none_expr,
        }
    };
}

#[macro_export]
macro_rules! stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);
