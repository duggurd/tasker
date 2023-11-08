use std::error::Error;
use std::iter::Cloned;
use std::{env::args, fs};
use std::fs::OpenOptions;
use std::io::{Write};

use doit::Doit;
use doit::MainCmd;

fn main() {
    let argv = args();
    let arg_s: Vec<String> = argv.collect();
    
    let main_cmd = MainCmd::from_str(&arg_s[1]);

    let mut doit = Doit::default();
    doit.parse();
    
    match main_cmd {
        MainCmd::Task => {
            if arg_s.len() <= 2 {
                panic!("Not enough arguments in task");
            } else {
                doit.add_task(arg_s[2].to_string(),  arg_s.get(3).cloned() );
            }
        }
        MainCmd::Stage => {println!("stagin task X")}
        MainCmd::Done => {println!("Done with task X")}
        MainCmd::List => {println!("Listing tasks")}
    }
}






