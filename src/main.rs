use rust_decimal::Decimal;
use serde::Deserialize;

fn main() {
    #[derive(Debug, Deserialize)]
    pub struct ToBeDeser {
        pub n: Decimal,
    }

    let r: ToBeDeser = serde_json::from_str(r#"{"n": 0.5}"#).unwrap();
    println!("{:?}", r);
}
