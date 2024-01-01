mod cli;
mod server;

use cli::{generate_html, get_dir_data, Data};
use server::{ContentType, HttpVersion, Request, Response, StatusCode};
use std::{fs::File, io::Read, net::TcpListener, process::exit};

use std::{fs, path::Path};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "HttpServer", about = "An example of using HttpServer")]
struct Arguments {
    #[structopt(
        short = "f",
        long,
        help = "Sets the directory/fold",
        default_value = "."
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

fn run_server(dir: &Vec<Data>) {
    let ipaddr = "127.0.0.1:8000";

    println!("Listening...");
    let listener = TcpListener::bind(ipaddr).expect("Listener error");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut request = Request::from(stream);

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
    println!("Path: {:?}", req.path);

    if req.path == "/" {
        println!("Process ROOT: {:?}",current_dir_data);
        return Some(Response::new(
            HttpVersion::Http2_0,
            ContentType::TextHtml,
            StatusCode::Ok,
            &generate_html(&current_dir_data),
        ));
    } 
    else {

        let req_path = req.path.clone().chars().skip(1).collect::<String>();
        for dir in current_dir_data.iter() {

            // println!("Dir.name: {:?}, req: {:?}",dir.name,req.path.clone().chars().skip(1).collect::<String>());
            if dir.name == req_path{
                
                if dir.is_file {
                    println!("File name: {:?}\n", dir.name);

                    let mut file = File::open(req_path).expect("Impossible find file");

                    let mut content = String::new();
                    file.read_to_string(&mut content).unwrap();

                    return Some(Response::new(
                        HttpVersion::Http2_0,
                        ContentType::TextHtml,
                        StatusCode::Ok,
                        &content,
                    ));

                } else {
                    println!("Directory name: {:?}\n", dir.name);
                    return Some(Response::new(
                        HttpVersion::Http2_0,
                        ContentType::TextHtml,
                        StatusCode::Ok,
                        &generate_html(
                            &get_dir_data("/".to_string()+&dir.name.clone()).unwrap()
                        ),
                    ));
                }

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
