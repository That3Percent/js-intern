A macro for interning JavaScript primitives.

Stores one copy of each distinct JavaScript primitive.
For example, ```js_intern!("string")``` evaluates to a ```&JsValue``` but only
does the translation from the utf-8 Rust string to the utf-16 JavaScript
string the first time the expression is evaluated. Furthermore, strings
are de-duplicated across the program. So, any time ```js_intern!("string")```
is used in the program, the same instance of the JavaScript string is used.
# Supported types
* ```&'static str``` Eg: ```js_intern!("str")```
* ```f64```, ```f32``` ```u8```, ```u16```, ```u32```, ```i8```, ```i16```, ```i32``` Eg: ```js_intern(1.0)```
* ```bool``` Eg: ```js_intern(true)```

# Related
If you like this, you may like these other crates by Zac Burns
* [soa-vec](https://github.com/That3Percent/soa-vec) A struct of arrays layout with a Vec of tuple API