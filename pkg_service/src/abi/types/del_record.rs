#[derive(Debug)]
pub struct DelKvPair {
    pub r#type: u8,
    pub del_id: i64,
}

pub enum DelType {
    Package = 1,
    Relation = 2,
    Category = 3,
}

// impl into u8 from DelType
impl From<DelType> for u8 {
    fn from(del_type: DelType) -> u8 {
        match del_type {
            DelType::Package => 1,
            DelType::Relation => 2,
            DelType::Category => 3,
        }
    }
}
