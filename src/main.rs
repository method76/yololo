#![windows_subsystem = "windows"]

#[macro_use]
extern crate serde_derive; // 1.0.117
extern crate serde; // 1.0.117
extern crate serde_json; // 1.0.59
extern crate actix_rt;
extern crate actix_web;
extern crate actix_multipart;
extern crate futures;
extern crate mime_guess;
extern crate rust_embed;
extern crate image;
extern crate tinyfiledialogs as tfd;

use jammdb::{DB, Data, Error as JamError, Bucket};
use actix_web::{body::Body, web, App, HttpRequest, HttpResponse, HttpServer, Result, Responder, Error, http::Method, web::Json};
use actix_multipart::Multipart;
use actix_form_data::{Field, Form, Value};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use futures::{StreamExt, TryStreamExt, future::Future};
use std::{env, borrow::Cow, sync::mpsc, thread, path::Path, fs, io::Write, str, ffi::OsString, ffi::OsStr, cell::RefCell};
use yololo::*;

mod components;

#[derive(RustEmbed)]
#[folder = "src/web"]
struct Asset;

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    url: String,
    labeled: bool,
}

fn assets(req: HttpRequest) -> HttpResponse {

    let path = if req.path() == "/" {
        // if there is no path, return default file
        "index.html"
    } else {
        // trim leading '/'
        &req.path()[1..]
    };

    // query the file from embedded asset with specified path
    match Asset::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let server = HttpServer::new(|| App::new()
        .route("/api/v1/images", web::get().to(components::routes::get_images))
        .route("/api/v1/files", web::get().to(components::routes::get_files))
        .route("*", web::get().to(assets)))
        .bind("127.0.0.1:8080")?
        .run();

    yololo::builder()
        .title("Yololo for Yolo by Bitflow")
        // .content(Content::Html(include_str!("web/index.html")))
        // .content(Content::Html(html))
        .content(Content::Url(format!("http://127.0.0.1:{}", 8080)))
        .size(1280, 760)
        .resizable(true)
        .debug(true)
        .user_data(0)
        .invoke_handler(invoke_handler)
        .run()
        .unwrap();

    Ok(())
}


fn invoke_handler(wv: &mut WebView<usize>, arg: &str) -> WVResult {

    if arg == "on_images_selected" {
        let path_raw = components::utils::get_value();
        match path_raw {
            None => { println!("none"); },
            Some(data) => {
                println!("data {}", &data);
                components::files::open_dir(&data, wv);
            }
        }
    } else if arg == "files" {
        /*
        println!("files invoked");
        let path_raw = components::utils::get_value();
        match path_raw {
            None => { println!("none"); },
            Some(data) => {
                let results = &components::files::scan_txt_files(&data);
                println!("results {:?}", results);
                //wv.eval(&format!("train_txt({:?})", serde_json::to_string(results).unwrap()));
                wv.eval(&format!("train_txt('{:?}', {})", results, results.len()));
            }
        }
        */
    } else if arg == "exit" {
        println!("exiting!");
        wv.exit();
    } else if arg == "open" {
        let filter = ["*.jpg"];
        let filter2 = "";
        match tfd::select_folder_dialog("Please select a folder", "") {
            Some(path) => {
                components::files::open_dir(&path, wv);
            },
            None => {},
        }
    } else if arg == "get_rect" {
        println!("get_rect");
        // wv.eval(&format!("draw_rect('{:?}', {})", results, results.len()));
    }
    Ok(())
}