use std::fs;
use std::time;
use std::process;

pub struct Config {
    pub path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let mut path = String::from("./");

        if args.len() > 1 {
            path = args[1].clone();
        } 

        Ok(Config { path })
    }
}


pub fn run(path: String) {
    let dir = fs::read_dir(path.clone()).unwrap_or_else(|err| {
            eprintln!("Error getting file/folder metadata `{err}`");
            process::exit(1);
        });

    for i in dir {
        let f_name = i.unwrap_or_else(|err| {
            eprintln!("Error getting file/folder metadata `{err}`");
            process::exit(1);
        }).path().display().to_string().replace(&path, "").replace("/", "");

        let f_path = format!("{path}/{f_name}");

        let mut f_type = "Folder";

        if fs::metadata(f_path.clone()).unwrap_or_else(|err| {
            eprintln!("Error getting file/folder metadata `{err}`");
            process::exit(1);
        }).is_file() {
            f_type = "File";
        }

        let mut f_created = time::SystemTime::elapsed(&fs::metadata(f_path.clone()).unwrap_or_else(|err| {
            eprintln!("Error getting file/folder metadata `{err}`");
            process::exit(1);
        }).created().unwrap_or_else(|err| {
            eprintln!("Error getting file/folder creation time `{err}`");
            process::exit(1);
        })).unwrap_or_else(|err| {
            eprintln!("Error getting file/folder creation time `{err}`");
            process::exit(1);
        }).as_secs_f64().to_string();

        let mut f_modified = time::SystemTime::elapsed(&fs::metadata(f_path.clone()).unwrap_or_else(|err| {
            eprintln!("Error getting file/folder metadata `{err}`");
            process::exit(1);
        }).modified().unwrap_or_else(|err| {
            eprintln!("Error getting file/folder modified time `{err}`");
            process::exit(1);
        })).unwrap_or_else(|err| {
            eprintln!("Error getting file/folder modified time `{err}`");
            process::exit(1);
        }).as_secs_f64().to_string();

        let mut f_accessed = time::SystemTime::elapsed(&fs::metadata(f_path.clone()).unwrap_or_else(|err| {
            eprintln!("Error getting file/folder metadata `{err}`");
            process::exit(1);
        }).accessed().unwrap_or_else(|err| {
            eprintln!("Error getting file/folder accessed time `{err}`");
            process::exit(1);
        })).unwrap_or_else(|err| {
            eprintln!("Error getting file/folder accessed time `{err}`");
            process::exit(1);
        }).as_secs_f64().to_string();

        f_created = convert_time_to_days(f_created);

        f_modified = convert_time_to_days(f_modified);

        f_accessed = convert_time_to_days(f_accessed);

        print!("{} \t \t {} \t \t {} \t \t {} \t \t {} \n", f_type, f_created, f_modified, f_accessed, f_name);
    }
}


fn convert_time_to_days(time: String) -> String {
    ((time.parse::<f64>().unwrap_or_else(|err| {
            eprintln!("Error parsing data `{err}`");
            process::exit(1);
        }) / 86400.0) as i64).to_string()
}