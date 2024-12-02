#[macro_export]
macro_rules! input {
    ($n:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/", "d", $n, ".txt"))
    };
}