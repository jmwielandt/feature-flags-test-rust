#[cfg(all(feature = "with_f1", feature = "without_f1"))]
compile_error!("can't enable 'with_f1' and 'without_f1' at the same time");

fn main() {
    println!("Hello, world!");
    #[cfg(any(feature = "with_f1", feature = "without_f1"))]
    do_stuff();
    #[cfg(feature = "optional_feature")]
    optional_code();
}

#[cfg(feature = "with_f1")]
fn do_stuff() {
    println!("hello");
}

#[cfg(feature = "without_f1")]
fn do_stuff() {
    println!("good bye");
}

#[cfg(feature = "optional_feature")]
fn optional_code() {
    println!("Optional feature enabled!");
}
