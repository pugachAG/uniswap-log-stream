use anyhow::Result;
use csv::Writer;
use std::fs::File;

use crate::data::SwapEvent;

pub struct Persister {
    writer: Writer<File>,
}

const FIELD_COUNT: usize = 8;
const CSV_HEADERS: [&str; FIELD_COUNT] = [
    "transaction",
    "sender",
    "recipient",
    "amount_0",
    "amount_1",
    "sqrt_price_x96",
    "liquidity",
    "tick",
];

impl Persister {
    pub fn create(file_path: &str) -> Result<Self> {
        let mut writer = Writer::from_path(file_path)?;
        writer.write_record(&CSV_HEADERS)?;
        Ok(Self { writer })
    }

    pub fn write(&mut self, event: &SwapEvent) -> Result<()> {
        // Manually serialize here instead of using serde derive
        // because it serializes I256 as hex instead of number
        let record: [String; FIELD_COUNT] = [
            format!("{:#0x}", event.transaction_hash),
            format!("{:#0x}", event.sender),
            format!("{:#0x}", event.recipient),
            event.amount_0.to_string(),
            event.amount_1.to_string(),
            event.sqrt_price_x96.to_string(),
            event.liquidity.to_string(),
            event.tick.to_string(),
        ];
        self.writer.write_record(&record)?;
        self.writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::str::FromStr;
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::{env, fs};

    use ethers::types::I256;

    use crate::data::SwapEvent;

    use super::Persister;

    #[test]
    fn basic_persister() {
        let file_name = format!(
            "basic_persister_test_{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros()
        );
        let file_path = env::temp_dir().join(file_name).to_str().unwrap().to_owned();
        let mut persister = Persister::create(&file_path).unwrap();
        let event = SwapEvent {
            sender: parse("0xe37e799d5077682fa0a244d46e5649f71457bd09"),
            recipient: parse("0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640"),
            amount_0: I256::from(7872350000i64),
            amount_1: I256::from(-3499187381949677088i64),
            sqrt_price_x96: parse("1670764545678477469943279858557158"),
            liquidity: 8309089253574408855,
            tick: "199139".parse().unwrap(),
            transaction_hash: parse(
                "0x6d72690d4c899b67ed9bba811bd6f7bcb4ebe8fa017681408038a7d2b2bd69a4",
            ),
        };
        persister.write(&event).unwrap();

        let expected_header =
            "transaction,sender,recipient,amount_0,amount_1,sqrt_price_x96,liquidity,tick";
        let expected_row = "0x6d72690d4c899b67ed9bba811bd6f7bcb4ebe8fa017681408038a7d2b2bd69a4,0xe37e799d5077682fa0a244d46e5649f71457bd09,0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640,7872350000,-3499187381949677088,7635699703802906846552847833155694326104,8309089253574408855,199139";
        let contents = fs::read_to_string(file_path).unwrap();
        assert_eq!(contents, format!("{expected_header}\n{expected_row}\n"));
    }

    fn parse<F>(s: &str) -> F
    where
        F: FromStr,
        <F as FromStr>::Err: Debug,
    {
        s.parse().unwrap()
    }
}
