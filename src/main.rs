use {
  clap::{Arg, Command},
  ngine::{OptionValue, parse_options_boolean, parse_options_number, parse_options_string, run},
  std::collections::HashMap,
  tokio::runtime::Runtime
};

fn main() {
    let matches = Command::new("ngined")
        .about("Daemon for running an N-Body simulation with network capabilities")
        .version("0.1.0")
        .author("Your Name <sgeos@hotmail.com>")
        .arg(Arg::new("name")
             .short('n')
             .long("name")
             .env("NAME")
             .value_name("NAME")
             .help("Sets the server name."))
        .arg(Arg::new("port")
             .short('p')
             .long("port")
             .env("PORT")
             .value_name("PORT")
             .help("Sets the port number for the server to listen on."))
        .arg(Arg::new("verbose")
             .short('v')
             .long("verbose")
             .env("VERBOSE")
             .value_name("VERBOSE")
             .help("Activates verbose output."))
        .get_matches();

    let mut options = HashMap::<&str, OptionValue>::new();
    let boolean_value_keys = ["verbose"];
    for key in boolean_value_keys {
        options.insert(key, parse_options_boolean(&matches, key));
    }
    let numerical_value_keys = ["port"];
    for key in numerical_value_keys {
        options.insert(key, parse_options_number(&matches, key));
    }
    let string_value_keys = ["name"];
    for key in string_value_keys {
        options.insert(key, parse_options_string(&matches, key));
    }

    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    rt.block_on(async {
        run(options).await;
    });
}

