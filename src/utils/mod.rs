#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        eprintln!($($arg)*);
        std::process::exit(1);
    })
}

#[macro_export]
macro_rules! solution {
    ($module:ident) => {
        use std::time::Instant;
        let start = Instant::now();
        $module::run();
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    };
}

#[macro_export]
macro_rules! input_handler {
    () => {
        {
            let path = file!().replace("mod.rs", "input.txt");
            std::fs::File::open(path).unwrap()
        }
    };
}

#[macro_export]
macro_rules! input_string {
    () => {
        {
            let mut file = input_handler!();
            let mut input = String::new();
            file.read_to_string(&mut input).unwrap();
            input

        }
    };
}