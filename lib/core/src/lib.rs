use std::collections::{HashMap};
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::mem::transmute;
use std::hash::{Hash};

thread_local!(
	static FLOAT_CACHE: Cacher<BitwiseFloat> = Cacher::new();
	static STRING_CACHE: Cacher<&'static str> = Cacher::new();
	static BOOL_CACHE: Cacher<bool> = Cacher::new(); // TODO: This is a bit overkill.
	// TODO: Include None. The first thought would be for Option<!> if that compiles with a simple js_intern!(None). wasm-bindgen treats this as undefined rather than null, so then should we.
);

#[doc(hidden)]
/// This is a private trait and not meant to be used.
pub trait CacheJsIntern__ {
	fn cache_js_intern__(self) -> *mut JsValue;
}

impl CacheJsIntern__ for f64 {
	fn cache_js_intern__(self) -> *mut JsValue {
		FLOAT_CACHE.with(|c| {
			c.cache(self.into())
		})
	}
}

impl CacheJsIntern__ for &'static str {
	fn cache_js_intern__(self) -> *mut JsValue {
		STRING_CACHE.with(|c| {
			c.cache(self)
		})
	}
}

impl CacheJsIntern__ for bool {
	fn cache_js_intern__(self) -> *mut JsValue {
		BOOL_CACHE.with(|c| {
			c.cache(self)
		})
	}
}

macro_rules! CacheForT64 {
	($t:ty) => {
		impl CacheJsIntern__ for $t {
			fn cache_js_intern__(self) -> *mut JsValue {
				(self as f64).cache_js_intern__()
			}
		}
	};
}

CacheForT64!(i8);
CacheForT64!(i16);
CacheForT64!(i32);
CacheForT64!(u8);
CacheForT64!(u16);
CacheForT64!(u32);
CacheForT64!(f32);

struct Cacher<T: Eq + Hash> {
	inner: RefCell<HashMap<T, *mut JsValue>>
}

impl<T: Eq + Hash> Cacher<T> {
	fn new() -> Cacher<T> {
		Cacher {
			inner: RefCell::default()
		}
	}
}

// Implementing Drop is probably overkill, since in eg: a browser, there
// is only one JavaScript engine. But, I can imagine a system that had
// multiple JavaScript engines. Perhaps one per thread.
impl<T: Eq + Hash> Drop for Cacher<T> {
	fn drop(&mut self) {
		// Ensure we free all the heap allocations from our boxes,
		// and drop the js values contained in them.
		for (_key, value) in self.inner.borrow_mut().drain() {
			unsafe { Box::from_raw(value); }
		}
	}
}

impl<T: Into<JsValue> + Eq + Hash + Copy> Cacher<T> {
	fn cache(&self, value: T) -> *mut JsValue {
		let mut map = self.inner.borrow_mut();
		// Note that if Cacher is ever used outside this crate, we would need to make
		// this function re-entrant, since T::into<JsValue> could execute arbitrary
		// code, this could get called by it, and the borrow_mut() would panic.
		// For now, it's only used on types for which this is not a problem.

		*map.entry(value).or_insert_with(move || {
			let js_value: JsValue = value.into();
			Box::into_raw(Box::new(js_value))
		})
	}
}


/// For the purposes of this crate, floats are equal if and only if their bit patterns
/// are equal, since we are only responsible for the idea of caching the transfer of
/// the float into JavaScript and not what float semantics mean.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
#[repr(transparent)]
struct BitwiseFloat(u64);

impl From<BitwiseFloat> for JsValue {
	fn from(value: BitwiseFloat) -> JsValue {
		JsValue::from_f64(value.into())
	}
}

impl From<f64> for BitwiseFloat {
	fn from(value: f64) -> Self {
		unsafe { transmute(value) }
	}
}

impl From<BitwiseFloat> for f64 {
	fn from(value: BitwiseFloat) -> Self {
		unsafe { transmute(value) }
	}
}

/// Stores one copy of each distinct JavaScript primitive.
/// For example, ```js_intern!("string")``` evaluates to a ```&JsValue``` but only
/// does the translation from the utf-8 Rust string to the utf-16 JavaScript
/// string the first time the expression is evaluated. Furthermore, strings
/// are de-duplicated across the program. So, any time ```js_intern!("string")```
/// is used in the program, the same instance of the JavaScript string is used.
///
/// # Supported types
/// * ```&'static str``` Eg: ```js_intern!("str")```
/// * ```f64```, ```f32```, ```u8```, ```u16```, ```u32```, ```i8```, ```i16```, ```i32``` Eg: ```js_intern(1.0)```
/// * ```bool``` Eg: ```js_intern(true)```
///
/// # Warning: This is intended to work for literals only. It may presently work on expressions,
/// but this is not an intended part of the API and will break in a future release.
#[macro_export]
macro_rules! js_intern {
	($value:expr) => {
		{
			use wasm_bindgen::JsValue;
			use $crate::CacheJsIntern__;
			thread_local!(
				static INTERN: *mut JsValue = $value.cache_js_intern__();
			);

			// A word about the safety here. We are dereferencing a pointer
			// of type *mut JsValue. At the address of the pointer is a JsValue
			// instance that is boxed. The JsValue is only freed when the thread
			// goes out of scope. JsValue does not implement Send, so we know
			// that the value cannot be used anywhere it's invalid.
			unsafe { &*INTERN.with(|i| i.clone()) }
		}
	};
}