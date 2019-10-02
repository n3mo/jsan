use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
use std::fs::File;
use std::env;
use serde_json::Value;

fn main() -> std::io::Result<()> {
    // Command line arguments
    let args: Vec<String> = env::args().collect();
    // The first arg is the name of the program, so we'll
    // skip to the second argument for the file name
    let input_file = &args[1];
    let json_fields = &args[2..];
    let num_fields = json_fields.len();
    
    // let f = File::open("data/tweets.json")?;
    let f = File::open(input_file)?;
    let reader = BufReader::new(f);
    let mut writer = BufWriter::new(io::stdout());

    // Work across the file line-by-line, using buffered reading
    // and writing
    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Parse the current line's JSON string
                let v: Value = serde_json::from_str(&line)?;

                // Parse and write each requested JSON field
                // for field in json_fields {
                for i in 0..num_fields {
                    writer.write(v[&json_fields[i]].to_string().as_bytes())
                        .expect("Error: Malformed JSON?");
                    if i < num_fields - 1 {
                        writer.write(b",").unwrap();
                    }
                }
                writer.write(b"\n").unwrap();
            },
            Err(e) => {
                println!("Err: {}", e);
            },
        }
    }

    Ok(())
}
