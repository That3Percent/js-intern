extern crate proc_macro;
use proc_macro::*;
use syn::{parse_macro_input, Expr};
use quote::{quote};


/// Attempts to determine if an expression evaluates to a const. This is in general impossible knowing only the AST without the context.
/// By way of example, we cannot know if a function call to an ident refers to a function which is a const fn. So, we prefer false negatives
/// in the interest of actually compiling.
// TODO: There are a great many more cases that this can work that should be added.
fn is_conservatively_const(expr: &Expr) -> bool {
	match expr {
		Expr::Lit(lit) => {
			lit.attrs.is_empty()
		}
		Expr::Group(group) => {
			is_conservatively_const(&group.expr)
		}
		_ => false,
	}
}

/// Does a very conservative check to see if an expression is a candidate for interning.
/// If the expression is a candidate to be interned this will return `js_intern!(...)`
/// Otherwise, the original expression is returned.
#[proc_macro]
pub fn try_js_intern(input: TokenStream) -> TokenStream {
	let orig = input.clone();
	let expr = parse_macro_input!(input as Expr);
	if is_conservatively_const(&expr) {
		quote!({
			use js_intern::js_intern;
			js_intern!(#expr)
		}).into()
	} else {
		orig
	}
}