use std::process::exit;

pub trait Rust<T, F> {
	fn rust(self, F) -> T;
}

impl<T, E, F> Rust<T, F> for Result<T, E>
	where E: ::std::error::Error,
		  F: Fn(E) -> ()
{
	fn rust(self, f: F) -> T {
		match self {
			Ok(s) => s,
			Err(e) => { f(e); exit(-1) }
		}
	}
}

impl<T, F> Rust<T, F> for Option<T> 
	where F: Fn() -> ()
{
	fn rust(self, f: F) -> T {
		match self {
			Some(s) => s,
			None => { f(); exit(-1) }
		}
	}
} 
