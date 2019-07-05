A macro for interning JavaScript primitives.

Stores one copy of each distinct JavaScript primitive.
For example, ```js_intern!("string")``` evaluates to a ```&JsValue``` but uses only one heap allocation
and a one-time translation from the utf-8 Rust string to the utf-16 JavaScript string the first time the expression is evaluated.
Furthermore, values are de-duplicated across the program.
So, any time ```js_intern!(1.0)``` is used in the program, the same instance of the JavaScript number is used.
# Supported types
* ```&'static str``` Eg: ```js_intern!("str")```
* ```f64```, ```f32``` ```u8```, ```u16```, ```u32```, ```i8```, ```i16```, ```i32``` Eg: ```js_intern(1.0)```
* ```bool``` Eg: ```js_intern(true)```

# Related
If you like this, you may like these other crates by Zac Burns (That3Percent)
* [js-object](https://github.com/That3Percent/js-object) A macro for creating JavaScript objects
* [soa-vec](https://github.com/That3Percent/soa-vec) A struct of arrays layout with a Vec of tuple API
* [second-stack](https://github.com/That3Percent/second-stack) A Rust memory allocator for large slices that don't escape the stack.