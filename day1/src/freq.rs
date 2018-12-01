#[derive(Debug)]
pub struct Freq {
    pub sign: Sign,
    pub value: i32,
}

#[derive(Debug)]
pub enum Sign {
    Plus,
    Minus,
}
