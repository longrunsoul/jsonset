use std::{
    fs::{
        File,
        OpenOptions,
    },
    str::FromStr,
};
use anyhow::Context;
use plainjson::JsonNode;
use std::io::Write;

mod cli;

fn main() {
    let cli_matches = cli::get_cli().get_matches();
    let key = cli_matches.value_of("key").unwrap();
    let value = cli_matches.value_of("value").unwrap();
    let file = cli_matches.value_of("FILE").unwrap();
    let as_null = cli_matches.is_present("as-null");
    let as_bool = cli_matches.is_present("as-bool");
    let as_number = cli_matches.is_present("as-number");

    let mut json;
    {
        let file = File::open(file).context("Failed to open file").unwrap();
        json = JsonNode::parse_single_node(file).context("Failed to parse json").unwrap();
    }

    if as_null {
        json.set_null(key).context("Failed to select nodes by specified JSONPath").unwrap();
    } else if as_bool {
        let b = bool::from_str(value).context("Failed to parse value as bool").unwrap();
        json.set_bool(key, b).context("Failed to select nodes by specified JSONPath").unwrap();
    } else if as_number {
        let n = f64::from_str(value).context("Failed to parse value as number").unwrap();
        json.set_number(key, n).context("Failed to select nodes by specified JSONPath").unwrap();
    } else {
        json.set_str(key, value).context("Failed to select nodes by specified JSONPath").unwrap();
    }

    let json_text = format!("{:#}", json);
    {
        let mut file =
            OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(file)
                .context("Failed to open file to write")
                .unwrap();
        file.write_all(json_text.as_bytes()).context("Failed to write file").unwrap();
    }
}
