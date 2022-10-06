use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    // active verbose mode
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    // file to generate
    #[structopt(short = "r", long = "result", parse(from_os_str))]
    result_file: PathBuf,

    // files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

// execute `cargo run input1.txt input2.txt -v --result res.xyz`
fn main() {
    println!("{:?}", Opt::from_args());
}
