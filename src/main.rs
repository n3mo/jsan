use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
use std::fs::File;
//use std::env;
use serde_json::Value;
use clap::{Arg, App};

fn main() -> std::io::Result<()> {

    // Command line parsing with clap
    let args = App::new("jsan -- The JSON Swiss Army kNife")
        .version("0.0.1")
        .author("Nicholas M. Van Horn (nvanhorn@protonmail.com)")
        .about("A fast JSON scraping tool")
        .arg(Arg::with_name("field")
             .short("f")
             .long("field")
             .help("JSON field(s) to scrape")
             .required(true)
             .min_values(1)
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
        .get_matches();

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

    let json_fields: Vec<&str> = args.values_of("field").unwrap().collect();
    let num_fields = json_fields.len();
    
    let reader = BufReader::new(rdr);
    // let mut writer = BufWriter::new(io::stdout());
    let mut writer = BufWriter::new(wtr);

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

                    let mut child = v.clone();
                    let mut child2 = child.clone();
                    
                    for idx in json_fields[i].split("::") {
                        child2.clone_from(&child[idx]);
                        child.clone_from(&child2);
                    }

                    writer.write(child.to_string().as_bytes())
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



