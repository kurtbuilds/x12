use x12::segments::ProfessionalService;
use serde_x12::from_str;

#[test]
fn test_sv1() {
    let s = "SV1*HC:96153:HF*50.1*UN*6***1:2~";
    let v: ProfessionalService = from_str(&s).unwrap();
}