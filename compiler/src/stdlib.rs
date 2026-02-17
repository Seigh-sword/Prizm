// Standard Library for Prizm

pub mod builtins {
    // File Operations
    pub mod file {
        pub fn create(path: &str) -> Result<(), String> {
            match std::fs::File::create(path) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to create file: {}", e)),
            }
        }

        pub fn delete(path: &str) -> Result<(), String> {
            match std::fs::remove_file(path) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to delete file: {}", e)),
            }
        }

        pub fn move_file(from: &str, to: &str) -> Result<(), String> {
            match std::fs::rename(from, to) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to move file: {}", e)),
            }
        }

        pub fn read(path: &str) -> Result<String, String> {
            match std::fs::read_to_string(path) {
                Ok(content) => Ok(content),
                Err(e) => Err(format!("Failed to read file: {}", e)),
            }
        }

        pub fn write(path: &str, content: &str) -> Result<(), String> {
            match std::fs::write(path, content) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to write file: {}", e)),
            }
        }
    }

    // Math Operations
    pub mod math {
        use rand::Rng;

        pub fn add(a: i64, b: i64) -> i64 {
            a + b
        }

        pub fn subtract(a: i64, b: i64) -> i64 {
            a - b
        }

        pub fn multiply(a: i64, b: i64) -> i64 {
            a * b
        }

        pub fn divide(a: i64, b: i64) -> Result<i64, String> {
            if b == 0 {
                Err("Division by zero".to_string())
            } else {
                Ok(a / b)
            }
        }

        pub fn modulo(a: i64, b: i64) -> Result<i64, String> {
            if b == 0 {
                Err("Modulo by zero".to_string())
            } else {
                Ok(a % b)
            }
        }

        pub fn random(min: i64, max: i64) -> i64 {
            let mut rng = rand::thread_rng();
            rng.gen_range(min..=max)
        }

        pub fn power(base: i64, exp: u32) -> i64 {
            base.pow(exp)
        }
    }

    // Output Operations
    pub mod output {
        pub fn print(text: &str) {
            print!("{}", text);
        }

        pub fn println(text: &str) {
            println!("{}", text);
        }
    }

    // String Operations
    pub mod string {
        pub fn length(s: &str) -> usize {
            s.len()
        }

        pub fn concat(a: &str, b: &str) -> String {
            format!("{}{}", a, b)
        }

        pub fn uppercase(s: &str) -> String {
            s.to_uppercase()
        }

        pub fn lowercase(s: &str) -> String {
            s.to_lowercase()
        }
    }
}