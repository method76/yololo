extern crate serde_derive;  // 1.0.117
extern crate serde;         // 1.0.117
extern crate serde_json;    // 1.0.59
extern crate actix_web;

use actix_web::{body::Body, web, App, HttpRequest, HttpResponse, HttpServer, Result, Responder, Error, http::Method, web::Json};
use std::{env, borrow::Cow, sync::mpsc, thread, path::Path, fs, io::Write, str, ffi::OsString, ffi::OsStr};
use serde::{Deserialize, Serialize};
use super::*;
use crate::components::structs::{ TextFile, Image };


pub async fn get_images(req: HttpRequest) -> Result<Json<Vec<Image>>> {

    println!("get_images");

    let mut ret:Vec<Image> = Vec::new();
    let path_raw = utils::get_value();

    return match path_raw {
        None => { Ok(Json(ret)) },
        Some(path) => {
            let paths = fs::read_dir(path).unwrap();
            let mut counter = 0;
            for path in paths {
                let name = path.unwrap().path().display().to_string();
                if name.to_lowercase().ends_with(".jpg") {
                    counter = counter + 1;
                    let mut name_no_ext = Path::new(&name).file_stem().unwrap().to_os_string();
                    &name_no_ext.push(&OsStr::new(".txt"));
                    let is_txt_exists = Path::new(&name).parent().unwrap().join(&name_no_ext).exists();
                    // println!("is_txt_exists {} {}", is_txt_exists, &name_no_ext.to_str().unwrap());
                    ret.push(Image { url: name.replace("\\", "/"), labeled: is_txt_exists });
                    if counter == 180 {
                        break;
                    }
                }
            }
            Ok(Json(ret))
        }
    }
}

pub async fn get_files(req: HttpRequest) -> Result<Json<Vec<TextFile>>> {

    println!("get_files");

    let path_raw = super::utils::get_value();

    match path_raw {
        None => { Ok(Json(Vec::new())) },
        Some(data) => {
            let ret = super::files::scan_txt_files(&data);
            // println!("results {:?}", ret);
            Ok(Json(ret))
        }
    }
}