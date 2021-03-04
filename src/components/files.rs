extern crate serde_json; // 1.0.59

use std::{env, borrow::Cow, sync::mpsc, thread, path::Path, fs, io::Write, str, ffi::OsString, ffi::OsStr};
use serde::{Deserialize, Serialize};
use super::*;


pub fn open_dir(path: &str, wv: &mut super::super::WebView<usize>) {
    // Save Path to Local Storage
    super::utils::save_path(path.clone());
    let results =  &scan_img_files(path);
    let winpath = path.clone().replace("\\", "\\\\");
    wv.eval(&format!("show_images({:?}, '{}')", serde_json::to_string(results).unwrap(), &winpath));
}

fn scan_img_files(path: &str) -> Vec<super::structs::Image> {
    let paths = fs::read_dir(path).unwrap();
    let mut filenames = vec![];
    let mut counter = 0;
    for path in paths {
        let name = path.unwrap().path().display().to_string();
        if name.to_lowercase().ends_with(".jpg") {
            counter = counter + 1;
            let mut name_no_ext = Path::new(&name).file_stem().unwrap().to_os_string();
            &name_no_ext.push(&OsStr::new(".txt"));
            let is_txt_exists = Path::new(&name).parent().unwrap().join(&name_no_ext).exists();
            // println!("is_txt_exists {} {}", is_txt_exists, &name_no_ext.to_str().unwrap());
            let item = super::structs::Image {
                url: name,
                labeled: is_txt_exists,
            };
            filenames.push(item);
            if counter == 180 {
                break;
            }
        }
    }
    println!("total: {}", counter);
    return filenames;
}

pub fn scan_txt_files(path: &str) -> Vec<super::structs::TextFile> {
    let paths = fs::read_dir(path).unwrap();
    let mut filenames = vec![];
    let mut counter = 0;
    for path in paths {
        let name = path.unwrap().path().display().to_string();
        if name.to_lowercase().ends_with(".jpg") {
            counter = counter + 1;
            let mut name_no_ext = Path::new(&name).file_stem().unwrap().to_os_string();
            &name_no_ext.push(&OsStr::new(".txt"));
            let txt_filename = Path::new(&name).parent().unwrap().join(&name_no_ext);
            let is_txt_exists = txt_filename.exists();
            if is_txt_exists == true {
                let item = super::structs::TextFile {
                    url: name, // txt_filename.into_os_string().into_string().unwrap(),
                    labeled: is_txt_exists,
                };
                filenames.push(item);
            }
        }
    }
    filenames
}

/*pub fn scan_txt_files(path: &str) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();
    let mut filenames = vec![];
    let mut counter = 0;
    for path in paths {
        let name = path.unwrap().path().display().to_string();
        if name.to_lowercase().ends_with(".jpg") {
            counter = counter + 1;
            let mut name_no_ext = Path::new(&name).file_stem().unwrap().to_os_string();
            &name_no_ext.push(&OsStr::new(".txt"));
            let txt_filename = Path::new(&name).parent().unwrap().join(&name_no_ext);
            let is_txt_exists = txt_filename.exists();
            if is_txt_exists == true {
                let txt_filename_str = txt_filename.into_os_string().into_string().unwrap();
                println!("filename {}", &name);
                filenames.push(name);
            }
        }
    }
    filenames
}*/
