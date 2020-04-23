#![deny(
//    missing_docs,
    missing_debug_implementations,
    bare_trait_objects,
    anonymous_parameters,
    elided_lifetimes_in_paths
)]
#![warn(
    clippy::all,
//    clippy::restriction,
//    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

pub mod services;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
