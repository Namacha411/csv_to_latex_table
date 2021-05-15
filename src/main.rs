use csv::Reader;
use std::collections::BTreeSet;
use std::{io, io::Stdin};
use structopt::{clap, StructOpt};

fn main() {
    let opt = Opt::from_args();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());

    write_table(&mut reader, &opt);
}

#[derive(Debug, StructOpt)]
#[structopt(name = "csv2tbl")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
    #[structopt(short, long, default_value = "htb")]
    pub table: String,

    #[structopt(short = "C", long)]
    pub caption: Option<String>,

    #[structopt(long)]
    pub caption_is_under: bool,

    #[structopt(short, long)]
    pub label: Option<String>,

    #[structopt(short, long)]
    pub column: String,

    #[structopt(short, long, default_value = "0")]
    pub hline: String,
}

fn write_table(reader: &mut Reader<Stdin>, opt: &Opt) {
    let set = &opt
        .hline
        .split(' ')
        .map(|s| s.trim())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<BTreeSet<usize>>();

    println!("\\begin{{table}}[{}]", &opt.table);
    if !opt.caption_is_under {
        if let Some(caption) = &opt.caption {
            println!("\t\\caption{{{}}}", caption);
        }
    }
    if let Some(label) = &opt.label {
        println!("\t\\label{{{}}}", label);
    }
    println!("\t\\begin{{tabular}}{{{}}}", &opt.column);

    for (i, result) in reader.records().enumerate() {
        print!("\t\t");
        let record = result.unwrap();
        let back = record.len() - 1;
        for i in 0..back {
            print!("{} & ", &record[i]);
        }
        print!("{} \\\\", &record[back]);
        if set.contains(&i) {
            print!(" \\hline");
        }
        println!();
    }

    println!("\t\\begin{{tabular}}");
    if opt.caption_is_under {
        if let Some(caption) = &opt.caption {
            println!("\t\\caption{{{}}}", caption);
        }
    }
    println!("\\end{{table}}");
}
