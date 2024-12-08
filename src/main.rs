#![feature(string_remove_matches)]

use std::{
    env::args,
    io::{Read, Write},
};

#[allow(dead_code)]
enum ItemType {
    TextFile,
    Directory,
    Error,
    Binary,
    Redundant,
    Information,
}

fn main() {
    let mut args = args();
    args.next();
    let mut url = args.next().unwrap();
    if !url.starts_with("gopher://") {
        url = "gopher://".to_owned() + &url;
    }
    let mut url = url::Url::parse(&url).unwrap();
    if url.port() == None {
        url.set_port(Some(70)).unwrap();
    }
    let path = url.path().to_string();
    url.set_path("");
    let mut url_str = url.to_string();
    url_str.remove_matches("gopher://");
    let mut stream = std::net::TcpStream::connect(url_str).unwrap();
    stream.write(path.as_bytes()).unwrap();
    stream.write("\r\n".as_bytes()).unwrap();
    let mut data_vec: Vec<u8> = Vec::new();
    stream.read_to_end(&mut data_vec).unwrap();
    let data = String::from_utf8(data_vec).unwrap();
    let items = data.split("\r\n");
    for item in items {
        if item == "." {
            break;
        }
        let type_char = item.chars().next().unwrap();
        let item_type = match type_char {
            '0' => ItemType::TextFile,
            '1' => ItemType::Directory,
            '3' => ItemType::Error,
            '5' | '9' => todo!("reading type Binary is unimplemented"),
            '+' => ItemType::Redundant,
            'i' => ItemType::Information,
            _ => unimplemented!("unknown item type {}", type_char),
        };
        let (_, t) = item.split_at(1);
        let main_item = t.split("\t").collect::<Vec<&str>>();
        match item_type {
            ItemType::TextFile => {
                print!("[text] ");
                print!("{}", main_item[0]);
                println!(
                    " [access \"{}:{}{}\" for this item]",
                    main_item[2], main_item[3], main_item[1]
                )
            }
            ItemType::Directory => {
                print!("[dir] ");
                print!("{}", main_item[0]);
                println!(
                    " [access \"{}:{}{}\" for this item]",
                    main_item[2], main_item[3], main_item[1]
                )
            }
            ItemType::Error => {
                print!("[error] ");
                println!("{}", main_item[0]);
                //println!(
                //    " [access \"{}:{}/{}\" for this item]",
                //    main_item[2], main_item[3], main_item[1]
                //)
            }
            ItemType::Binary => unreachable!(),
            ItemType::Information => {
                println!("{}", main_item[0]);
            }
            ItemType::Redundant => {
                print!("[also] ");
                print!("{}", main_item[0]);
                println!(
                    " [access \"{}:{}{}\" for this item]",
                    main_item[2], main_item[3], main_item[1]
                )
            }
        }
    }
}
