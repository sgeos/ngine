use {
    clap::ArgMatches,
    std::{collections::HashMap, io::Write},
};

#[derive(Debug)]
pub struct OptionValue {
    pub text: String,
    pub boolean_value: bool,
    pub numerical_value: i32,
    pub parse_error: bool,
}

impl OptionValue {
    fn new(text: &str) -> Self {
        let boolean_value = false; // Example of a default boolean value
        let numerical_value = 0; // Default numerical value
        let parse_error = false; // Default parse error status

        OptionValue {
            text: text.to_string(),
            boolean_value,
            numerical_value,
            parse_error,
        }
    }
}

// Function to parse a specific command-line option into an OptionValue
pub fn parse_options_string(matches: &ArgMatches, key: &str) -> OptionValue {
    if let Some(value) = matches.get_one::<String>(key) {
        OptionValue::new(value)
    } else {
        OptionValue {
            text: "".to_string(),
            boolean_value: false,
            numerical_value: 0,
            parse_error: true, // Indicate a parse error if the key is not found
        }
    }
}

// Parses a boolean option and sets numerical_value based on the boolean result
pub fn parse_options_boolean(matches: &ArgMatches, key: &str) -> OptionValue {
    match matches.get_one::<String>(key) {
        Some(value) => {
            let (boolean_value, parse_error) = match value.to_lowercase().as_str() {
                "true" | "yes" | "1" => (true, false),
                "false" | "no" | "0" => (false, false),
                _ => (false, true),
            };
            OptionValue {
                text: value.to_string(),
                boolean_value,
                numerical_value: if boolean_value { 1 } else { 0 },
                parse_error,
            }
        },
        None => OptionValue {
            text: String::new(),
            boolean_value: false,
            numerical_value: 0,
            parse_error: true,
        },
    }
}

// Parses a numerical (i32) option and sets boolean_value based on the numerical result
pub fn parse_options_number(matches: &ArgMatches, key: &str) -> OptionValue {
    match matches.get_one::<String>(key) {
        Some(value) => match value.parse::<i32>() {
            Ok(numerical_value) => OptionValue {
                text: value.to_string(),
                boolean_value: numerical_value != 0,
                numerical_value,
                parse_error: false,
            },
            Err(_) => OptionValue {
                text: value.to_string(),
                boolean_value: false,
                numerical_value: 0,
                parse_error: true,
            },
        },
        None => OptionValue {
            text: String::new(),
            boolean_value: false,
            numerical_value: 0,
            parse_error: true,
        },
    }
}

pub async fn run(options: HashMap<&'static str, OptionValue>) {
    println!("Server starting with options: {:?}", options);

    // Placeholder for future functionality
    println!("Placeholder for n-body simulation and network code");

    // Simulate some asynchronous operation
    let timeout = 5;
    for _ in 0..timeout {
      print!(". ");
      let _ = std::io::stdout().flush();
      tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    println!();

    println!("Server shutting down...");
}


