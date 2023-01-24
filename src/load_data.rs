use calamine::{open_workbook, Reader, Xlsx};
use lazy_static::{__Deref, lazy_static};

use crate::config::TALENTS_DATA_PATH;

#[derive(Clone, Debug)]
pub struct Talent {
    pub name: String,
    pub desc: String,
    _id: i32,
}

impl Talent {
    pub fn display(&self) -> String {
        format!("{}: {}", self.name, self.desc)
    }
}

lazy_static! {
    pub static ref ALL_TALENTS: Vec<Talent> = load_talents().expect("load talents failed");
    pub static ref ALL_EVENTS: Vec<String> = vec![
        "我精神状况挺好的啊。".to_owned(),
        "我没事不用担心我哈。".to_owned()
    ];
}

fn load_talents() -> anyhow::Result<Vec<Talent>> {
    let mut excel: Xlsx<_> = open_workbook(TALENTS_DATA_PATH.deref())?;
    let talents: Vec<Talent> = if let Some(Ok(r)) = excel.worksheet_range("talents") {
        r.rows()
            .skip(2)
            .map(|row| Talent {
                name: row[1].to_string(),
                desc: row[2].to_string(),
                _id: row[0]
                    .to_string()
                    .parse::<i32>()
                    .unwrap_or_else(|_| panic!("wrong talent id? {}", row[0]))
                    as i32,
            })
            .collect()
    } else {
        return Err(anyhow::Error::msg("no talents?"));
    };
    Ok(talents)
}
