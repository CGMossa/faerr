use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

#[extendr]
fn foo(some_mat: RMatrix<f64>) -> Mat<f64> {
    let some_mat_faer = Mat::<f64>::from(some_mat);

    return some_mat_faer
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod faerr;
    fn hello_world;
    fn foo;
}
