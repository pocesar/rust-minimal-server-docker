use std::{
    fs::read_dir,
    path::PathBuf,
    collections::HashSet,
};
use clap::Parser;
use glob::glob;


#[derive(Parser)]
struct Options {
    #[clap(long)]
    path: Option<String>,
    #[clap(long)]
    pattern: Option<String>,
    #[clap(long)]
    address: Option<String>,
    #[clap(long)]
    port: Option<u16>,
}

#[derive(Clone)]
struct FileServer {
    base_path: PathBuf,
    pattern: String,
    file_paths: HashSet<String>,
}

impl FileServer {
    fn new(base_path: PathBuf, pattern: String) -> Self {
        Self {
            base_path,
            pattern,
            file_paths: HashSet::new(),
        }
    }

    pub fn init(&mut self) {
        self.walk_dir(&self.base_path.clone());
    }

    fn walk_dir(&mut self, path: &PathBuf) {
        let pattern_path = path.join(&self.pattern);
        let file_paths = glob(&pattern_path.to_string_lossy());

        if let Ok(paths) = file_paths {
            for v in paths
                .filter_map(|x| x.ok())
                .into_iter() {
                    if v.is_dir() {
                        self.walk_dir(&v);
                    } else if v.is_file() {
                        self.file_paths.insert(v.canonicalize().unwrap().to_string_lossy().to_string());
                    }
            }
        }
    }

    pub fn count(&self) -> usize {
        self.file_paths.len()
    }
}

async fn serve_file(req: tide::Request<FileServer>) -> tide::Result {
    let path_param = req.param("path");
    let base_path = &req.state().base_path;
    let file_paths = &req.state().file_paths;

    if let Ok(requested_file) = path_param {
        let normalized_path = PathBuf::from(&base_path).join(requested_file).canonicalize();

        if let Ok(normalized_path) = normalized_path {
            let path_found = file_paths.get(&normalized_path.to_string_lossy().to_string());

            if let Some(path_found) = path_found {
                let mut response = tide::Response::new(200);
                response.set_body(tide::Body::from_file(path_found).await?);
                return Ok(response);
            }
        }
    }

    let response = tide::Response::new(404);
    Ok(response)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let options = Options::parse();

    let path: PathBuf = options.path.unwrap_or_else(|| "/serve".into()).into();
    _ = read_dir(&path).expect("Failed to read directory");

    let pattern = options.pattern.unwrap_or_else(|| "*".into());
    let host = options.address.unwrap_or_else(|| "127.0.0.1".into());
    let port = options.port.unwrap_or_else(|| 8080);

    let listen_address = format!("{}:{}", host, port);

    let mut file_server = FileServer::new(path, pattern);
    file_server.init();

    println!("Serving {} files on {}", &file_server.count(), &listen_address);
    let mut app = tide::with_state(file_server);

    app.at("*path").get(serve_file);

    app.listen(listen_address).await?;

    Ok(())
}
