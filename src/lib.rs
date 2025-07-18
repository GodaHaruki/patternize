pub mod builder {
    pub use patternize_internals::builder::Builder;
    pub use patternize_macros::generate_builder;
}

pub mod singleton {
    pub use patternize_macros::generate_singleton;
}
