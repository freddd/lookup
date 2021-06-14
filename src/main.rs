use std::{io::{self, BufRead}, net::IpAddr};

use clap::{App, Arg};
use prettytable::{Cell, Row, Table};
use trust_dns_resolver::{Resolver, config::{ResolverConfig, ResolverOpts}, proto::rr::Record};
use serde::{Deserialize, Serialize};


const HOST: &str = "host";
const IP: &str = "ip";

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
struct Lookup {
    kind: String,
    name: String,
    value: String,
    state: String,
}

impl Lookup {
    fn from(record: &Record) -> Lookup {
        Lookup {
            kind: record.rr_type().to_string(),
            name: record.name().to_string(),
            value: record.rdata().to_string(),
            state: "SUCCESS".to_string(),
        }
    }

    fn failed(subject: String) -> Lookup {
        Lookup {
            state: "FAILED".to_string(),
            kind: "".to_string(),
            name: subject.clone(),
            value: subject,
        }
    }
}

fn main() {
    let matches = App::new("lookup")
    .version("1.0")
    .author("freddd")
    .about("ip to hostname or hostname to ip")
    .arg(
        Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("table, json, limited output")
            .default_value("table")
            .env("LOOKUP_OUTPUT"),
    )
    .arg(
        Arg::with_name(HOST)
            .short("h")
            .long(HOST)
            .required(false)
            .takes_value(false)
            .help(HOST)
    )
    .arg(
        Arg::with_name(IP)
            .short("ip")
            .long(IP)
            .required(false)
            .takes_value(false)
            .help(IP)
    )
    .arg(
        Arg::with_name("subjects")
            .global(true)
            .min_values(1)
            .multiple(true),
    )
    .get_matches();

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let subjects: Vec<String>  = match matches.values_of("subjects") {
        Some(values) => values.map(|ln| ln.to_string()).collect(),
        None => io::stdin().lock().lines().map(|ln| ln.unwrap()).collect(),
    };

    let lookups: Vec<Lookup> = match matches.is_present(HOST) {
        true => { 
            subjects
            .iter()
            .map(|s| {
                match resolver.lookup_ip(s.as_str()) {
                    Ok(lookup) => {
                        lookup.as_lookup()
                        .record_iter()
                        .map(|r| Lookup::from(r))
                        .collect::<Vec<Lookup>>()
                    },
                    Err(_) => {
                        vec![Lookup::failed(s.to_string())]
                    },
                }
            })
            .flatten()
            .collect::<Vec<Lookup>>()
        }
        false => { 
            subjects
            .iter()
            .map(|s| {
                match resolver.reverse_lookup(s.parse::<IpAddr>().expect("failed to parse ip")) {
                    Ok(lookup) => {
                        lookup.as_lookup()
                        .record_iter()
                        .map(|r| Lookup::from(r))
                        .collect::<Vec<Lookup>>()
                    },
                    Err(_) => {
                        vec![Lookup::failed(s.to_string())]
                    },
                }
            })
            .flatten()
            .collect::<Vec<Lookup>>()
        }
    };

    match matches.value_of("output").unwrap() {
        "json" => println!("{}", serde_json::to_string(&lookups).unwrap()),
        "table" => print_as_table(lookups),
        "limited" => lookups.iter().for_each(|l| println!("{}", l.value)),
        _ => unreachable!(),
    }
}

fn print_as_table(lookups: Vec<Lookup>) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("KIND"),
        Cell::new("NAME"),
        Cell::new("VALUE"),
        Cell::new("STATE"),
    ]));

    for lookup in lookups {
        table.add_row(Row::new(vec![
            Cell::new(&lookup.kind),
            Cell::new(&lookup.name),
            Cell::new(&lookup.value),
            Cell::new(&lookup.state),
        ]));
    }

    table.printstd();
}
