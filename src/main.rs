// src/main.rs
use std::{self, env};
use tokio;
use tokio_util;

use clap::{Command, Arg};

use axum::{
    routing::get,
    Router, extract::Path, response::{IntoResponse, Html},
    body::Body, http::header
};


#[tokio::main]
async fn main() {
    let args = Command::new("app")
        .version("1.0")
        .author("rocyin")
        .about("file download server")
        .arg(Arg::new("ip").long("ip").required(true))
        .arg(Arg::new("port").long("port").required(true))
        .arg(Arg::new("serve_dir").long("serve_dir").required(true))
        .get_matches()
        ;

    let ip = args.get_one::<String>("ip").unwrap();
    let port = args.get_one::<String>("port").unwrap();
    let ip_and_port = format!("{ip}:{port}");

    let serve_dir = args.get_one::<String>("serve_dir").unwrap();
    env::set_current_dir(serve_dir).expect("set work dir err");

    let app = Router::new()
        .route("/", get(root_processor))
        .route("/*path", get(processor));

    let listener = tokio::net::TcpListener::bind(&ip_and_port).await.unwrap();
    println!("server is running, serving on {ip_and_port}");
    axum::serve(listener, app).await.unwrap();
}


// pub struct PathAndMeta {
//     path: String,
// }

async fn root_processor() -> impl IntoResponse {

    let root_dir = std::env::current_dir().unwrap();
    let entries = std::fs::read_dir(&root_dir).unwrap();
    let mut lines = vec![];
    for entry in entries {
        let entry = entry.unwrap();
        entry.metadata().unwrap().modified().unwrap()
        let filename = entry.file_name().to_str().unwrap().to_string();
        let prefix = if entry.file_type().unwrap().is_dir() {
            "DIR :"
        } else {
            "FILE:"
        };
        lines.push(format!("{} <a href=/{}> {} </a>", prefix, &filename, &filename));
        // lines.push(entry.path().to_str().unwrap().to_string());
    }
    lines.sort();
    Html(lines.join("<br/>"))

}

async fn processor(Path(pt): Path<String>) -> impl IntoResponse {
    println!("pt:{}", &pt);
    let root_dir = std::env::current_dir().unwrap();
    let curent_path = root_dir.join(&pt);
    if curent_path.is_file() {
        return {
            // 下载逻辑
            let file = tokio::fs::File::open(curent_path.to_str().unwrap()).await.unwrap();
            let stream = tokio_util::io::ReaderStream::new(file);
            let body = Body::from_stream(stream);

            let headers = [
                (header::CONTENT_TYPE, "text/plain; charset=utf-8".to_string()),
                (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", curent_path.file_name().unwrap().to_str().unwrap())),
            ];

            (headers, body)
        };
    }

    // 处理目录嵌套
    if curent_path.is_dir() {
        let entries = std::fs::read_dir(&curent_path).unwrap();
        let mut lines = vec![];
        for entry in entries {
            let entry = entry.unwrap();
            let filename = entry.file_name().to_str().unwrap().to_string();
            let prefix = if entry.file_type().unwrap().is_dir() {
                "DIR :"
            } else {
                "FILE:"
            };
            lines.push(format!("{} <a href=/{}/{}> {} </a>", prefix, &pt, &filename, &filename));
        }
        lines.sort();
        let res_content = lines.join("<br/>");

        return (
            [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string()),
            (header::SERVER, "axum".to_string()),
            ],
            Body::from(res_content)
        );
    }

    (
        [(header::CONTENT_TYPE, "text/html; charset=utf-8".to_string()),
        (header::SERVER, "axum".to_string()),
        ],
        Body::from(format!("invalid path: {}", curent_path.to_str().unwrap()))
    )
}
