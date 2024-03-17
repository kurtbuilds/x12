use serde_x12::{detect_format, from_str};
use x12::Document;
use x12::release_5010::transactions::HealthCareClaim;

#[test]
fn test_sv1() {
    let s = include_str!("../../data/CHPW_Claimdata.txt");
    let f = detect_format(s).unwrap();
    let v: Document<HealthCareClaim> = from_str(&s).unwrap();
    let r = f.to_string(&v).unwrap();
    assert_eq!(s, r);
}