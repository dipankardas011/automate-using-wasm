#[allow(warnings)]
mod bindings;

use std::env;
use clap::Parser;
use std::fs;
use std::time::{Duration, SystemTime};
use std::thread::sleep;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CommandToPerform {
    #[arg(short = 'n', long = "name")]
    name: String,

    #[arg(short='o', long="op", value_parser=[OP_GENERATE_RANDOM_PASSWORD, OP_PROJ_LATEST_RELEASE, "demo"], default_value_t=String::from("demo"))]
    operation: String,
}


const FILE_PATH: &str = "README.md";
const OP_GENERATE_RANDOM_PASSWORD: &str = "gen_rand_pass";
const OP_PROJ_LATEST_RELEASE: &str = "pro_latest_release";


use bindings::dipankardas011::{ crypto::password::generate_random, githubapi::releases::fetch_latest };

use reqwest;

async fn fetch_latest_internal(org: &str, proj: &str) {
    let url = format!("https://api.github.com/repos/{org}/{proj}/releases/latest");
    println!("Url: {url}");
    let body = reqwest::get(url).await;

    match body {
        Ok(b) => {
            let bb = b.text().await;
            println!("body = {bb:?}");
        }
        Err(e) => {
            println!("Err: {e}");
        }
    }
}

async fn hh() {
    println!(" @@ Called Async GG @@");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = CommandToPerform::parse();

    hh().await;

    if args.operation == OP_GENERATE_RANDOM_PASSWORD {
        println!(" > Enter Length of Password");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let length_pass: u32 = input.trim().parse().expect("Invalid Input");

        let gen_pass = generate_random(length_pass);

        println!("Created password '{gen_pass}'");

    } else if args.operation == OP_PROJ_LATEST_RELEASE {
        println!(" > Enter Organization Name");
        let mut input_org = String::new();
        std::io::stdin().read_line(&mut input_org).expect("Failed to read line");
        let org: String = input_org.trim().parse().expect("invalid organization");

        println!(" > Enter Repo Name");
        let mut input_proj = String::new();
        std::io::stdin().read_line(&mut input_proj).expect("Failed to read line");
        let proj: String = input_proj.trim().parse().expect("invalid organization");

        let ver = fetch_latest(&org, &proj);
        println!("Latest version: {ver}");
        fetch_latest_internal(&org, &proj).await;
        
    } else {
        println!("Your Name: {}, Op: {}", args.name, args.operation);

        for (key, value) in env::vars() {
            println!("{key} : {value}");
        }

        println!("In file {FILE_PATH}");

        let contents =
            fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

        println!("Content upto 50 chars: {}\n", &contents[..50]);

        let now = SystemTime::now();

        sleep(Duration::new(2, 0));
        match now.elapsed() {
            Ok(elapsed) => {
                println!("Sleeped for {}s", elapsed.as_secs());
            }
            Err(e) => {
                println!("Error: {e:?}");
            }
        }
    }
}

