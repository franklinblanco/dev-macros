
macro_rules! test_macro {
    ($out_struct:path, $path:literal) => ( {
        println!($path);
    });
}

fn _s() {
    test_macro!(String, "a");
}