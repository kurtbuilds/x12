#![allow(unused)]

use std::fs;
use std::str::FromStr;

use serde_x12::{detect_format, detect_version, Element, from_str, to_string};
use x12::Document;
use x12::release_5010::transactions::HealthCareClaim;
use x12::InterchangeControlHeader;

fn main() {
    // let el = Element::Container(vec!["hello".to_string()]);
    // let s = serde_json::to_string(&el).unwrap();
    // eprintln!("Element: {}", s);
    // let x12 = include_str!("../data/CHPW_Claimdata.txt");
    let x12 = include_str!("/Users/kurt/Downloads/HCHC.APC.20220325PC029");
    eprintln!("{}", x12);

    let f = detect_format(x12).unwrap();
    let doc = from_str::<Document<HealthCareClaim>>(&x12).unwrap();
    let r = f.to_string(&doc).unwrap();
    // println!("{}", x12);
    assert_eq!(x12, r);
    // fs::write("output.txt", x12).unwrap();
}