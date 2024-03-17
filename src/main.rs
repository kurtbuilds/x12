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
    let x12 = include_str!("../data/CHPW_Claimdata.txt");
    let x12 = include_str!("/Users/kurt/Downloads/HCHC.APC.20220325PC029");
    // let doc = edi::parse(x12).unwrap();
    // dbg!(&doc);
    let v = detect_format(x12);
    dbg!(v);
    eprintln!("{:?}", &x12[106..108]);
    unimplemented!();
    let segments: Vec<_> = x12.split("\n").collect();
    // let lines = segments.len();
    // dbg!(&segments[..lines]);
    // let x12 = segments[..lines].join("\n") + "\n";
    let doc = from_str::<Document<HealthCareClaim>>(&x12).unwrap();
    // eprintln!(" === Completed parsing ===");
    // eprintln!("Header: {:#?}", rt_doc.functional_group_header);
    // for tx in &rt_doc.transactions {
    //     for l in &tx._2000 {
    //         for l in &l._2300 {
    //             for l in &l._2400 {
    //                 eprintln!("{:?}", l.professional_service);
    //             }
    //         }
    //     }
    // }
    //     for r in &tx.reference {
    //         eprintln!("Reference: {:#?}", r);
    //     }
    //     for l in &tx._1000 {
    //         eprintln!("Entity: {:#?}", l);
    //     }
    //     for l in &tx._2000 {
    //         // eprintln!("Provider: {:?}", l.provider);
    //         // eprintln!("Patient: {:?}", l.patient);
    //         for l in &l._2010 {
    //             eprintln!("Demographic: {:#?}", l.demographic);
    //         }
    //         for l in &l._2300 {
    //             for l in &l._2320 {
    //                 eprintln!("Demographic: {:#?}", l.demographic);
    //             }
    //             eprintln!("Claim: {:#?}", l.health_claim);
    //         }
    //     }
    // }
    let x12 = to_string(&doc).unwrap();
    println!("{}", x12);
    fs::write("output.txt", x12).unwrap();
}