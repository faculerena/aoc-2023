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
        {
        use std::time::Instant;
        let start = Instant::now();
        let sol = $module::run1();
        let sol2 = $module::run2();
        let duration = start.elapsed();
        println!("{}: {:?}", stringify!($module).replace("day", "Day "), duration);
        println!("Solution Part 1: \n\t {}", sol);
        println!("Solution Part 2: \n\t {}", sol2);
        sol
        }
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

#[macro_export]
macro_rules! get_input {
    () => {
        {
            let path = file!().replace("mod.rs", "input.txt");
            let folder = std::path::Path::new(&path).parent().unwrap().to_str().unwrap().to_string().replace("src/solutions/day", "");
            let cookie = std::fs::read_to_string(std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(".cookie")).unwrap();
            let url = format!("https://adventofcode.com/2023/day/{}/input", folder);

            let mut cmd = std::process::Command::new("curl");
            cmd.arg("--cookie").arg(cookie).arg(url).arg("-o").arg(path);
            cmd.output().unwrap();
            input_string!()
        }
    };
}


