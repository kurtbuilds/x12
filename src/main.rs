use serde_x12::{detect_format, from_str};
use x12::Document;
use x12::release_5010::transactions::HealthCareClaim;

fn main() {
    let x12 = include_str!("../data/CHPW_Claimdata.txt");
    let f = detect_format(x12).unwrap();
    let doc = from_str::<Document<HealthCareClaim>>(&x12).unwrap();
    let r = f.to_string(&doc).unwrap();
    assert_eq!(x12, r);
}
