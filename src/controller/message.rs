#[derive(Copy, Clone, Debug)]
pub enum Message {
    None,
    Changed(bool),
    PrevPressed,
    NextPressed,
}
