use derive_more::Constructor;

#[derive(Debug, Constructor)]
pub struct UnexpectedChar {
    pub value: String,
    pub index: usize,
}
