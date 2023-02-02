use std::fs;

use lazy_static::lazy_static;

pub const DEFAULT_WIN_WIDTH: u32 = 500;
pub const DEFAULT_WIN_HEIGHT: u32 = 800;
pub const TITLE: &str = "人生重开模拟器";

lazy_static! {
    pub static ref DEFAULT_FONT: Vec<u8> = fs::read::<String>(format!(
        "{}/resource/SourceHanSansCN-Regular.otf",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("get font resouce failed");
    pub static ref TALENTS_DATA_PATH: String = {
        println!("CARGO_MANIFEST_DIR={}", env!("CARGO_MANIFEST_DIR"));
        format!("{}/data/zh-cn/talents.xlsx", env!("CARGO_MANIFEST_DIR"))
    };
}
