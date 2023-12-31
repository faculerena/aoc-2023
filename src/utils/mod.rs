#[macro_export]
macro_rules! solution {
    ($module:ident) => {{
        use std::time::Instant;

        let start = Instant::now();
        let sol = $module::run1();
        let duration1 = start.elapsed();

        let start2 = Instant::now();
        let sol2 = $module::run2();
        let duration2 = start2.elapsed();

        println!(
            "{} {}: {:?}",
            stringify!($module).replace("day", "Day "),
            "part 1",
            duration1
        );
        println!("Solution Part 1: \n\t {:?}", sol);

        println!(
            "{} {}: {:?}",
            stringify!($module).replace("day", "Day "),
            "part 2",
            duration2
        );
        println!("Solution Part 2: \n\t {:?}", sol2);
        (sol, sol2)
    }};
}

#[macro_export]
macro_rules! input_handler {
    () => {{
        let path = file!().replace("mod.rs", "input.txt");
        std::fs::File::open(path).unwrap()
    }};
}

#[macro_export]
macro_rules! input_string {
    () => {{
        let path = file!().replace("mod.rs", "input.txt");
        let mut file = std::fs::File::open(path).unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        input
    }};
}

#[macro_export]
macro_rules! input_lines {
    () => {{
        input_string!().lines()
    }};
}

#[macro_export]
macro_rules! test_input {
    () => {{
        let path = file!().replace("mod.rs", "test.txt");
        let mut file = std::fs::File::open(path).unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        input
    }};
}
