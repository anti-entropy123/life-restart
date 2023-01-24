#[derive(Debug, Clone)]
pub enum Message {
    Loaded,
    Start,
    ClickTalentItem(usize),
    SubmitTalent,
    PressEventBox,
    GameOver,
}
