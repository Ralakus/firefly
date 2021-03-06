/// Prints using debug writer.
#[macro_export]
macro_rules! print {
	() => ({});
    ($($args:tt)+) => {{
		use core::fmt::Write;
        let _ = write!($crate::arch::debug::Writer::default(), $($args)*);
	}};
}

/// Prints line using debug writer.
#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}
