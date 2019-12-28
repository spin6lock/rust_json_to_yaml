use std::fs::File;
use std::io::Read;
use std::str;

extern crate clap;
use clap::{Arg, App};
use serde_json::{Value};

fn parse_args() -> String {
    let matches = App::new("json to yaml convertor")
                        .version("0.1.0")
                        .author("spin6lock")
                        .about("read json file from args and convert to yaml format")
                        .arg(Arg::with_name("file")
                             .short("f")
                             .long("json")
                             .value_name("FILE")
                             .help("a json file that small enough to fit in memory")
                             .required(true))
                        .get_matches();
    matches.value_of("file").unwrap().to_string()
}

fn main() -> std::io::Result<()> {
    let filename = parse_args();
    let mut fh = File::open(filename)?;
    let mut buffer = Vec::new();
    fh.read_to_end(&mut buffer)?;
    let s = str::from_utf8(&buffer).unwrap();
    let v: Value = serde_json::from_str(s)?;
    let yaml_result = serde_yaml::to_string(&v).unwrap();
    println!("{}", yaml_result);
    Ok(())
}
