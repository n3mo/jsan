use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
use std::fs::File;
//use std::env;
use serde_json::Value;
use clap::{Arg, App};

fn main() -> std::io::Result<()> {

    // Command line parsing with clap
    let args = App::new("jsan -- The JSON Swiss Army kNife")
        .version("0.1.0")
        .author("Nicholas M. Van Horn (nvanhorn@protonmail.com)")
        .about("A fast JSON scraping tool")
        .arg(Arg::with_name("key")
             .short("k")
             .long("key")
             .help("JSON key(s) to scrape")
             .required(true)
             .min_values(1)
             .takes_value(true))
        .arg(Arg::with_name("delimeter")
             .short("d")
             .long("delimeter")
             .help("Delimeter string (default = \",\")")
             .max_values(1)
             .takes_value(true))
        .arg(Arg::with_name("input")
             .short("i")
             .long("input")
             .help("Optional input file (else stdin)")
             .takes_value(true))
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .help("Optional output file (else stdout)")
             .takes_value(true))
        .arg(Arg::with_name("noheader")
             .long("noheader")
             .help("Suppress column name headers")
             .takes_value(false))
        .get_matches();

    // Delimeter sequence
    // let delimeter = match args.is_present("delimeter") {
    //     true => args.value_of("delimeter").to_string().as_bytes(),
    //     false => b",",
    // };
    let delimeter = args.value_of("delimeter").unwrap_or(",");

    // If the user has specified an input file, read from
    // it. Otherwise, read from stdin
    let rdr: Box<dyn io::Read> = match args.is_present("input") {
        true => Box::new(File::open(args.value_of("input").unwrap()).unwrap()),
        false => Box::new(io::stdin()),
    };

    // If the user has specified an output file, write to
    // it. Otherwise, write to stdin
    let wtr: Box<dyn io::Write> = match args.is_present("output") {
        true => Box::new(File::create(args.value_of("output").unwrap()).unwrap()),
        false => Box::new(io::stdout()),
    };

    let json_keys: Vec<&str> = args.values_of("key").unwrap().collect();
    let num_keys = json_keys.len();
    
    let reader = BufReader::new(rdr);
    // let mut writer = BufWriter::new(io::stdout());
    let mut writer = BufWriter::new(wtr);

    // Write column headers to file?
    match args.is_present("noheader") {
        false => {
            for i in 0..num_keys {
                writer.write(json_keys[i].as_bytes()).expect("Bad JSON key");
                if i < num_keys - 1 {
                    writer.write(delimeter.as_bytes()).unwrap();
                }
            }
            writer.write(b"\n").unwrap();
        },
        true => (),
    }

    // Work across the file line-by-line, using buffered reading
    // and writing
    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Parse the current line's JSON string
                let v: Value = serde_json::from_str(&line)?;

                // Parse and write each requested JSON key
                // for key in json_keys {
                for i in 0..num_keys {

                    let mut child = v.clone();
                    let mut child2 = child.clone();
                    
                    for idx in json_keys[i].split("::") {
                        child2.clone_from(&child[idx]);
                        child.clone_from(&child2);
                    }

                    writer.write(child.to_string().as_bytes())
                        .expect("Error: Malformed JSON?");
                    if i < num_keys - 1 {
                        writer.write(delimeter.as_bytes()).unwrap();
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



