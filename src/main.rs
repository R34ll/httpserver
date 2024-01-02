mod cli;
mod server;

use cli::{generate_html, get_dir_data, Data};
use server::{ContentType, HttpVersion, Request, Response, StatusCode};
use std::{fs::File, io::Read, net::TcpListener, process::exit};
use chrono::{Local, DateTime, NaiveDate};
use std::{fs, path::Path};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "HttpServer", about = "An example of using HttpServer")]
struct Arguments {
    #[structopt(
        short = "f",
        long,
        help = "Sets the directory/fold",
        default_value = "/"
    )]
    fold: String,

    #[structopt(short, long, help = "Sets the port", default_value = "8000")]
    port: u16,

    #[structopt(
        short,
        long,
        help = "Enable debug mode",
        parse(try_from_str),
        default_value = "true"
    )]
    debug: bool,
}

// 01/01/2024 | 127.0.0.1 | HTTP/2.0 | GET /test 200
pub fn middleware(req: &Request){    

    let local: DateTime<Local> = Local::now();
    let date= local.date_naive();

    let s = format!("\n| {} | {:?} | {:?} | {:?} | {:?} | {:?} |",date, req.host, req.version, req.method, req.path, "2000");

    println!("{}",s);
}



fn run_server(dir: &Vec<Data>) {
    let ipaddr = "127.0.0.1:8000";

    println!("Listening...");
    let listener = TcpListener::bind(ipaddr).expect("Listener error");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut request = Request::from(stream);
                middleware(&request);

                let response = process_request(&request, &dir);

                match response {
                    Some(res) => request.response(res),
                    None => request.response(Response::new(
                        HttpVersion::Http2_0,
                        ContentType::TextHtml,
                        StatusCode::Ok,
                        "Not find",
                    )),
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn process_request(req: &Request, current_dir_data: &Vec<Data>) -> Option<Response> {

    println!("Path: {:?} ", req.path);

    if req.path == "/" {
        println!("Process ROOT: ");
        return Some(Response::new(
            HttpVersion::Http2_0,
            ContentType::TextHtml,
            StatusCode::Ok,
            &generate_html(&current_dir_data),
        ));
    } 
    else {

        let req_path = req.path.clone().chars().skip(1).collect::<String>(); // req.path=="/<content>" | req_path="<content>"
        
        for dir in current_dir_data.iter(){

            if dir.is_file && dir.name == req_path.clone(){
                let mut file = File::open(req_path.clone()).expect("Impossible find file");
                let mut content = String::new();
                file.read_to_string(&mut content).expect("was not possible get the data content");

                return Some(Response::new(
                    HttpVersion::Http1_1, 
                    ContentType::TextHtml, 
                    StatusCode::Ok, 
                    &content
                ))

            }
            
            if dir.is_file == false && dir.name == req_path.clone(){
                let new_dir_data = get_dir_data(req_path.clone());

                return Some(Response::new(
                    HttpVersion::Http1_1, 
                    ContentType::TextHtml, 
                    StatusCode::Ok, 
                    &generate_html(&new_dir_data.expect("Impossible uwnrap"))
                ))

            }
        }

    }


    None
}

fn main() {
    let opt = Arguments::from_args();

    let current_dir_data = get_dir_data(opt.fold).unwrap();

    run_server(&current_dir_data);
}
