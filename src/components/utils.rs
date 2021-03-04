extern crate jammdb;
extern crate image;

use jammdb::{DB, Data, Error as JamError, Bucket};
use std::{env, borrow::Cow, sync::mpsc, thread, path::Path, fs, io::Write, str, ffi::OsString, ffi::OsStr};
use image::io::Reader as ImageReader;

pub fn get_value() -> Option<String> {
    let db_raw = DB::open("./db/jam.db");
    match db_raw {
        Err(err) => { None },
        Ok(db) => {
            let mut tx = db.tx(true).unwrap();
            let bucket_raw = tx.get_bucket("yololo");
            match bucket_raw {
                Err(err) => { None },
                Ok(bucket) => {
                    if let Some(data) = bucket.get(b"dir") {
                        let ret = str::from_utf8(data.kv().value().clone()).to_owned().unwrap();
                        Some(ret.parse().unwrap())
                    } else {
                        None
                    }
                },
            }

        }
    }
}

pub fn save_path(path: &str) {
    let db_raw = DB::open("./db/jam.db");
    match db_raw {
        Err(err) => { },
        Ok(db) => {
            let tx = db.tx(true).unwrap();
            let bucket_result = tx.get_bucket("yololo");
            let mut bucket;
            if bucket_result.is_err() {
                bucket = tx.create_bucket("yololo").unwrap();
            } else {
                bucket = bucket_result.unwrap();
            }
            bucket.put(b"dir", path.as_bytes());
            tx.commit();
        },
    }
}

pub fn conv_to_thumb(org_img_path: String) -> () {

    let src_path = Path::new(&org_img_path);
    let filename = src_path.file_name().unwrap();
    let image = ImageReader::open(src_path).unwrap().decode().unwrap();
    // 제일 마지막 문자가 .PNG, pNg, .Png 등
    let dest_path = src_path.parent().unwrap().parent().unwrap().join("target").join(
        filename.to_os_string().into_string().unwrap().replace(".png", ".jpg"));
    println!("dest_path {}", dest_path.display());
    image.save(dest_path).unwrap();

}