#[allow(unused_macros)]
/// The `fb!` macro (Flexi Block) *or* (Function Builder) simplifies the generation of conditional synchronous or asynchronous functions within Rust code.
///
/// It's mainly used alongside flexi_func ff proc macro to mark the sync & async parts of your code.
/// By specifying a mode, function name, parameters, return type, and body, this macro can dynamically create the desired function type based on the provided mode.
/// This approach is particularly useful in contexts where both synchronous and asynchronous versions of a function might be needed, allowing for cleaner code organization and reuse.
///
/// # Syntax
///
/// ```
/// fb!(mode, function_name, (parameter1: Type1, parameter2: Type2, ...), -> ReturnType, {
///     // Function body
/// });
/// ```
///
/// # Parameters
///
/// - `mode`: A compile-time direct token that determines whether the generated function is synchronous (`sync`) or asynchronous (`async`).
/// - `function_name`: The identifier for the generated function.
/// - `parameters`: A comma-separated list of function parameters in the form `parameter_name: Type`.
/// - `ReturnType`: The return type of the function.
/// - `body`: The block of code that defines the function body.
///
/// # Usage
///
/// Generating a synchronous function:
///
/// ```
/// fb!(sync, greet, (name: String), -> String, {
///     format!("Hello, {}", name)
/// });
/// ```
///
/// Generating an asynchronous function:
///
/// ```
/// fb!(async, fetch_data, (url: String), -> Result<String, reqwest::Error>, {
///     reqwest::get(&url).await?.text().await
/// });
/// ```
///
/// # Tricks and Advanced Usage
///
/// ## Conditional Compilation
///
/// The `fb!` macro can be combined with Rust's conditional compilation features to selectively compile either the synchronous or asynchronous version of a function based on feature flags or target environment.
///
/// Example with feature flags:
///
/// ```
/// #[cfg(feature = "async")]
/// fb!(async, process_data, (data: Vec<u8>), -> Result<(), MyError>, {
///     // Asynchronous processing
/// });
///
/// #[cfg(not(feature = "async"))]
/// fb!(sync, process_data, (data: Vec<u8>), -> Result<(), MyError>, {
///     // Synchronous processing
/// });
/// ```
///
/// ## Leveraging Macros for DRY Principles
///
/// You can define a wrapper macro around `fb!` to reduce repetition when declaring similar functions in different modes. This is especially handy when you have a set of functions that need to be available in both synchronous and asynchronous forms.
///
/// Example:
///
///
///  macro_rules! define_greeting_fn {
///     (sync) => {
///         fb!(sync, greet, (name: String), -> String, {
///             format!("Hello, {}", name)
///         });
///     };
///     (async) => {
///         fb!(async, greet, (name: String), -> String, {
///             format!("Hello, {}", name)
///         });
///     };
///   }
///
///
/// // Now, you can easily generate both versions with minimal repetition:
/// define_greeting_fn!(sync);
/// define_greeting_fn!(async);
/// ```
///
/// By leveraging the `fb!` macro in your Rust projects, you can maintain cleaner and more maintainable codebases, especially when dealing with the complexities of synchronous and asynchronous programming patterns.

// Improved macro definition for clarity and simplicity
#[macro_export]
macro_rules! fb {
    // Pattern for async function definition
    (async, $fn_name:ident, ($($param_name:ident : $param_type:ty),*), -> $return_type:ty, $body:block) => {
        async fn $fn_name($($param_name : $param_type),*) -> $return_type $body
    };
    // Pattern for sync function definition
    (sync, $fn_name:ident, ($($param_name:ident : $param_type:ty),*), -> $return_type:ty, $body:block) => {
        fn $fn_name($($param_name : $param_type),*) -> $return_type $body
    };
    // Pattern for returning an async closure
    (async, closure, $body:block) => {
        || async move $body
    };
    // Pattern for returning a sync closure
    (sync, closure, $body:block) => {
        || $body
    };
    // Pattern for immediate execution of an async block
    (async, execute, $body:block) => {
        async move $body
    };
    // Pattern for immediate execution of a sync block
    (sync, execute, $body:block) => {
        $body
    };
}
