//! Stores one copy of each distinct JavaScript primitive.
//! For example, ```js_intern!("string")``` evaluates to a ```&JsValue``` but only
//! does the translation from the utf-8 Rust string to the utf-16 JavaScript
//! string the first time the expression is evaluated. Furthermore, strings
//! are de-duplicated across the program. So, any time ```js_intern!("string")```
//! is used in the program, the same instance of the JavaScript string is used.
//!
//! # Supported types
//! * ```&'static str``` Eg: ```js_intern!("str")```
//! * ```f64```, ```f32```, ```u8```, ```u16```, ```u32```, ```i8```, ```i16```, ```i32``` Eg: ```js_intern(1.0)```
//! * ```bool``` Eg: ```js_intern(true)```
//!
//! # Related
//! If you like this, you may like these other crates by Zac Burns
//! * [soa-vec](https://github.com/That3Percent/soa-vec) A struct of arrays layout with a Vec of tuple API

pub use js_intern_core::js_intern;

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
}