#![allow(dead_code, unused_imports, unused_variables)] // TODO: disable
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    // missing_docs, // TODO: enable
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]
#![forbid(unsafe_code)]

fn main() {
    println!("Hello, {{project-name}}!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
