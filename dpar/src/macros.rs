#[macro_export]
macro_rules! ok_or_continue {
    ($expr:expr) => (match $expr {
        Some(val) => val,
        None => continue,
    })
}

#[macro_export]
macro_rules! ok_or_break {
    ($expr:expr) => (match $expr {
        Some(val) => val,
        None => break,
    })
}

#[macro_export]
macro_rules! try_ok {
    ($expr:expr) => (match $expr {
        Some(val) => val,
        None => return None,
    })
}

#[macro_export]
macro_rules! stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);
