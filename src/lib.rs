//! Stores one copy of each distinct JavaScript primitive.
//! For example, ```js_intern!("string")``` evaluates to a ```&JsValue``` but uses only one heap allocation
//! and a one-time translation from the utf-8 Rust string to the utf-16 JavaScript string the first time the expression is evaluated.
//! Furthermore, strings are de-duplicated across the program.
//! So, any time ```js_intern!(1.0)``` is used in the program, the same instance of the JavaScript number is used.
//!
//! # Supported types
//! * ```&'static str``` Eg: ```js_intern!("str")```
//! * ```f64```, ```f32```, ```u8```, ```u16```, ```u32```, ```i8```, ```i16```, ```i32``` Eg: ```js_intern(1.0)```
//! * ```bool``` Eg: ```js_intern(true)```
//!
//! # Related
//! If you like this, you may like these other crates by Zac Burns (That3Percent)
//! * [js-object](https://github.com/That3Percent/js-object) A macro for creating JavaScript objects
//! * [soa-vec](https://github.com/That3Percent/soa-vec) A struct of arrays layout with a Vec of tuple API
//! * [second-stack](https://github.com/That3Percent/second-stack) A memory allocator for large slices that don't escape the stack.
pub use js_intern_core::js_intern;
pub use js_intern_proc_macro::try_js_intern;

#[cfg(test)]
mod tests {
	use super::*;
	use wasm_bindgen_test::*;

	#[wasm_bindgen_test]
	fn can_convert_f64() {
		assert_eq!(Some(20.0), js_intern!(20.0).as_f64());
		assert!(js_intern!(std::f64::NAN).as_f64().unwrap().is_nan());
	}

	#[wasm_bindgen_test]
	fn can_convert_int() {
		assert_eq!(Some(1.0), js_intern!(1).as_f64());
		assert!(js_intern!(std::f64::NAN).as_f64().unwrap().is_nan());
	}

	#[wasm_bindgen_test]
	fn deduplicates_f64() {
		assert_eq!(js_intern!(15.0) as *const _, js_intern!(15.0) as *const _);
	}

	#[wasm_bindgen_test]
	fn can_convert_str() {
		assert_eq!(js_intern!("b").as_string(), Some(String::from("b")));
	}

	#[wasm_bindgen_test]
	fn deduplicates_str() {
		assert_eq!(js_intern!("a") as *const _, js_intern!("a") as *const _);
	}

	#[wasm_bindgen_test]
	fn can_convert_bool() {
		assert_eq!(js_intern!(true).as_bool(), Some(true))
	}

	#[wasm_bindgen_test]
	fn deduplicates_bool() {
		assert_eq!(js_intern!(true) as *const _, js_intern!(true) as *const _);
	}

	// TODO: It would be nice to have tests around try_js_intern, but that would require enabling the proc_macro_hygiene feature,
	// but I'm not sure what effect that would have on crates which would rely only on js_intern and not try_js_intern if they
	// would also need to upgrade to nightly
	/*
	#[wasm_bindgen_test]
	fn try_deduplicates_str_lit() {
		assert_eq!(try_js_intern!("a") as *const _, js_intern!("a") as *const _);
	}
	*/

}