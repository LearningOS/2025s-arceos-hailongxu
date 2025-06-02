//! Standard library macros

#[allow(unused)]
#[macro_export]
macro_rules! with_green_color {
    ($($arg:tt)*) => {{
        format_args!("\x1B[32m{}\x1B[0m", format_args!($($arg)*))
    }};
}

/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// [`println!`]: crate::println
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!($($arg)*));
        // $crate::io::__print_impl(with_green_color!("{}",format_args!($($arg)*)));
    }
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!("\x1B[32m{}\x1B[0m\n", format_args!($($arg)*)));
    }
}
