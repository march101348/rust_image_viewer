#[derive(Copy, Clone, Debug)]
pub enum Message {
    None,
    Changed,
    PrevPressed,
    NextPressed,
    FileNamePressed(usize),
}
