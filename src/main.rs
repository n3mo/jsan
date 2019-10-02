use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
use std::fs::File;
//use std::env;
use serde_json::Value;
use clap::{Arg, App};

// fn main() -> std::io::Result<()> {

//     // Command line parsing with clap
//     let args = App::new("jsan -- The JSON Swiss Army kNife")
//         .version("0.0.1")
//         .author("Nicholas M. Van Horn (nvanhorn@protonmail.com)")
//         .about("A fast JSON scraping tool")
//         .arg(Arg::with_name("field")
//              .short("f")
//              .long("field")
//              .help("Desired JSON fields to scrape")
//              .required(true)
//              .takes_value(true))
//         .arg(Arg::with_name("input")
//              .short("i")
//              .long("input")
//              .help("Sets the input file to use")
//              .takes_value(true))
//         .get_matches();

//     // Command line arguments
//     //let args: Vec<String> = env::args().collect();

//     // The first arg is the name of the program, so we'll
//     // skip to the second argument for the file name
//     // let input_file = &args[1];
//     // let json_fields = &args[2..];
//     // let num_fields = json_fields.len();

//     let input_file: &str = match args.is_present("input") {
//         true => args.value_of("input").unwrap(),
//         false => args.value_of("input").unwrap(),
//     };

    
//     // let json_fields = &args[2..];
//     // let num_fields = json_fields.len();
//     let json_fields: Vec<&str> = args.values_of("field").unwrap().collect();
//     let num_fields = json_fields.len();
    
//     // let f = File::open("data/tweets.json")?;
//     let f = File::open(input_file)?;
//     let reader = BufReader::new(f);
//     let mut writer = BufWriter::new(io::stdout());

//     // Work across the file line-by-line, using buffered reading
//     // and writing
//     for line in reader.lines() {
//         match line {
//             Ok(line) => {
//                 // Parse the current line's JSON string
//                 let v: Value = serde_json::from_str(&line)?;

//                 // Parse and write each requested JSON field
//                 // for field in json_fields {
//                 for i in 0..num_fields {
//                     writer.write(v[&json_fields[i]].to_string().as_bytes())
//                         .expect("Error: Malformed JSON?");
//                     if i < num_fields - 1 {
//                         writer.write(b",").unwrap();
//                     }
//                 }
//                 writer.write(b"\n").unwrap();
//             },
//             Err(e) => {
//                 println!("Err: {}", e);
//             },
//         }
//     }


//     Ok(())
// }
fn main() -> std::io::Result<()> {

    // Command line parsing with clap
    let args = App::new("jsan -- The JSON Swiss Army kNife")
        .version("0.0.1")
        .author("Nicholas M. Van Horn (nvanhorn@protonmail.com)")
        .about("A fast JSON scraping tool")
        .arg(Arg::with_name("field")
             .short("f")
             .long("field")
             .help("Desired JSON fields to scrape")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("input")
             .short("i")
             .long("input")
             .help("Sets the input file to use")
             .takes_value(true))
        .get_matches();

    // If the user has specified an input file, read from
    // it. Otherwise, read from stdin
    let rdr: Box<dyn io::Read> = match args.is_present("input") {
        true => Box::new(File::open(args.value_of("input").unwrap()).unwrap()),
        false => Box::new(io::stdin()),
    };

    let json_fields: Vec<&str> = args.values_of("field").unwrap().collect();
    let num_fields = json_fields.len();
    
    let reader = BufReader::new(rdr);
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
