use x12::release_5010::segments::Subscriber;
use serde_x12::{from_str, to_string};

#[test]
fn test_sbr() {
    let s = "SBR*P*18**COMMUNITY HLTH PLAN OF WASH*****CI~";
    let v: Subscriber = from_str(&s).unwrap();
    dbg!(&v);
    let r = to_string(&v).unwrap();
    assert_eq!(s, r.trim());
}