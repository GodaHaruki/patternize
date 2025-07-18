use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// Generate Builder struct, implementation for Builder struct, Builder trait implementation for Builder struct from provided struct
///
/// - Generated Builder name will be `XXXBuilder`.
/// - use `with_xxx` method to set value.
/// - use build method to get `Option<provided struct>` .
/// - `#[generate_builder(Traits)]` Traits will be derived to Builder struct by `#[derive(Traits)]`
/// - if there is data has not been set, build method returns `None`
/// - generated struct field will be `provided_field_name: Option<provided_field_type>`
///
/// # Examples
/// ```
/// # extern crate patternize;
/// use patternize::builder::generate_builder;
///
/// #[derive(Debug, PartialEq)] // for assert_eq!
/// #[generate_builder(Clone)]
/// struct Example {
///     data1: usize,
///     data2: isize
/// }
///
/// fn main() {
///     let example_builder: ExampleBuilder = ExampleBuilder::new()
///     .with_data1(0);
///     
///     assert_eq!(example_builder.clone().build(), None);
///     
///     let example: Option<Example> = example_builder.with_data2(-1).build();
///     
///     assert_eq!(example, Some(Example { data1: 0, data2: -1 }));
/// }
/// ```
/// # Generation Image
/// ```ignore
/// #[generate_builder(Debug, Clone)]
/// struct Example {
///     data: T
///     ...
/// }
///
/// // after generate
/// #[derive(Debug, Clone)]
/// struct ExampleBuilder {
///     data: Option<T>
///     ...
/// }
///
/// impl ExampleBuilder {
///     fn new() -> Self;
///     fn build(self) -> Example;
///     fn with_data(self, T) -> Self;
///     ...
/// }
///
/// impl patternize::builder::Builder for ExampleBuilder;
/// ```
#[proc_macro_attribute]
pub fn generate_builder(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr: proc_macro2::TokenStream = parse_macro_input!(attr);
    let input: DeriveInput = parse_macro_input!(input);
    patternize_internals::builder::generate_builder_derive(attr, input).into()
}

/// Generate singleton static variable
///
/// - generated variable name will be `SCREAMING_CAMEL_CASE` of `InputName`
/// - `InputName` must be written in `PascalCase`
/// - `#[generate_singleton]` will generate `std::sync::OnceLock<InputType>`
/// - `#[generate_singleton(expr)` will generate `std::sync::LazyLock<InputType>`
/// - `expr` type must be `InputType`
///
/// # Examples
/// ```
/// # extern crate patternize;
/// use patternize::singleton::generate_singleton;
///
/// #[derive(Debug, PartialEq)]
/// #[generate_singleton]
/// struct SampleSingleton {
///     data: std::string::String
/// }
///
/// #[derive(Debug, PartialEq)]
/// #[generate_singleton(SampleSingleton2 { data: "sample".to_string() })]
/// struct SampleSingleton2 {
///     data: std::string::String
/// }
///
/// fn main(){
///     // SAMPLE_SINGLETON: std::sync::OnceLock<SampleSingleton>
///     SAMPLE_SINGLETON.set(SampleSingleton {data: "sample".to_string()});
///     
///     assert_eq!(SAMPLE_SINGLETON.get(), Some(&SampleSingleton { data: "sample".to_string() }));
///
///     // SAMPLE_SINGLETON2: std::sync::LazyLock<SampleSingleton2>
///
///     assert_eq!(&*SAMPLE_SINGLETON2, &SampleSingleton2 { data: "sample".to_string() });
/// }
/// ```
#[proc_macro_attribute]
pub fn generate_singleton(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr: proc_macro2::TokenStream = parse_macro_input!(attr);
    let input: DeriveInput = parse_macro_input!(input);
    patternize_internals::singleton::generate_singleton_derive(attr, input).into()
}
