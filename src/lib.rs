/// Uses a thread local cache for the interned JsValue::from_str.
/// Note that the intern is just for the given expression, and is not global.
/// That may change in the future though.
#[macro_export]
macro_rules! js_intern {
	($value:expr) => {
		{
			// TODO: Use the special variable $crate https://medium.com/@kimond/how-to-use-external-crates-with-our-macros-in-rust-6dfe025351e0
			// to keep a hashmap to Pinned JsValue. An intern can look up or insert from there, and keep a threadlocal
			// pointer around as the result. We can then cast that to reference since JsValue is not Send, preventing it from escaping the thread.
			use wasm_bindgen::JsValue;
			thread_local!(
				static INTERN: JsValue = $value.into();
			);

			INTERN.with(|i| i.clone())
		}
	};
}