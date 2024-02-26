use clap::Parser;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Error},
    path::PathBuf,
};

/* Do not modify this */
// CLI args parser
#[derive(Parser, Debug)]
pub struct Args {
    // Csv file read path
    #[arg(short, long)]
    pub read_path: PathBuf,
    // Sub command for handling data in csv file
    #[clap(subcommand)]
    pub command: Command,
}

/* Do not modify this */
// Command and args prser
#[derive(Parser, Debug)]
pub enum Command {
    // Display entire file
    Display,
    // Modify a row/field
    Replace {
        #[clap(short, long)]
        row: usize,

        #[clap(short, long)]
        col: usize,

        // the new item to put into csv file
        #[clap(short, long)]
        data: String,
    },
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct CSVFile {
    data: Vec<Vec<String>>,
    rows: usize,
    cols: usize,
}

impl CSVFile {
    pub fn display(&self) {
        for row in &self.data {
            println!("{}", row.join(", "))
        }
    }

    pub fn modify(&mut self, row: usize, col: usize, new_val: String) {
        if row < self.data.len() && col < self.data[row].len() {
            self.data[row][col] = new_val
        }
    }

    pub fn read_input(&self, row:L) -> Command {
        let mut buf = String::new();

        io::stdin().read_line(&mut buf).expect("unable to read line");

        let input: usize = buf.trim().parse().unwrap_or(0)

        match command {
            1 => Command::Display,
            2 => Command::Replace
        }


    }



}

pub trait CSVFileReader {
    fn read(&mut self, file_path: PathBuf) -> Result<(), Error>;
}

impl CSVFileReader for CSVFile {
    fn read(&mut self, file_path: PathBuf) -> Result<(), Error> {
        let file = File::open(file_path)?;
        let buff = BufReader::new(file);

        for (index, line) in buff.lines().enumerate() {}

        Ok(())
    }
}

fn main() {
    // Reading CLI args
    let args = Args::parse();

    // create CSVFile instance and read file into it
    let mut csv = CSVFile::default();
    let _ = csv.read(args.read_path);

    // match and execute command
    match args.command {
        Command::Display => {
            println!("--Display CSVFile--")
            csv.display()
        },
        Command::Replace { row, col, data } => println!("--Replace and write to file--"),
    }
}
