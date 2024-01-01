use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Data {
    pub name: String,
    pub size: u64,
    pub is_file: bool,
}

impl Data {
    fn new(name: String, size: u64, file: bool) -> Self {
        Self {
            name,
            size,
            is_file: file,
        }
    }
}

pub fn get_dir_data(c_r_s: String) -> std::io::Result<Vec<Data>> {
    println!("{:?}",(c_r_s == ".") || (c_r_s == "/"));
    
    let c_r: PathBuf = if (c_r_s == ".") || (c_r_s == "/"){
        println!("ROOT FOLD");
        std::env::current_dir()?
    } else {

        let c_r = std::env::current_dir()?;
        let full_path = c_r.to_str().unwrap().to_string() + &c_r_s;
        let c_r = Path::new(&full_path);
        std::env::set_current_dir(&c_r)?;
        PathBuf::from(c_r)
    };


    println!("C_R_S: {:?} - Directory: {:?}",c_r_s,c_r.clone());

    let entries = fs::read_dir(c_r)?;
    let mut vec_data: Vec<Data> = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        // Get metadata
        let metadata = fs::metadata(path)?;

        vec_data.push(Data::new(
            entry.file_name().to_string_lossy().to_string(),
            metadata.len(),
            metadata.is_file(),
        ));
    }

    Ok(vec_data)
}

pub fn generate_html(data: &Vec<Data>) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html><html lang='en'><head><meta charset='UTF-8'><meta name='viewport' content='width=device-width, initial-scale=1.0'><title>HttpServer listening</title>");
    html.push_str("<style>table{border-collapse: collapse;width: 50%;margin: 20px;} th, td{border: 1px solid black;padding: 8px;text-align: left;}</style></head>");
    html.push_str("<body><h1>Files in localhost</h1><br>");
    html.push_str("<table><tr><th>Name</th> <th>Type</th> <th>Size(kb)</th> </tr>");

    for entry in data.iter() {
        if entry.is_file {
            html.push_str(
                format!(
                    "<tr><th><a href='{}'>{}</a></th> <th>{}</th> <th>{}</th> </tr>",
                    entry.name, entry.name, "File", entry.size
                )
                .as_str(),
            );
        } else {
            html.push_str(
                format!(
                    "<tr><th><a href='{}'>{}/</a></th> <th>{}</th> <th>{}</th> </tr>",
                    entry.name, entry.name, "Fold", entry.size
                )
                .as_str(),
            );
        }
    }
    html.push_str("</table></body></html>");
    html
}
