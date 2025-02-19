use serde::Serialize;

#[derive(Debug)]
pub enum ObjectData {
    EnumData(EnumData),
    Fields(Vec<Field>)
}

pub trait ListFields {
    fn list_fields() -> ObjectData;
}

#[derive(Serialize, Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: String
}

#[derive(Debug)]
pub struct EnumData {
    pub name: String,
    pub variants: Vec<Variant>
}

#[derive(Debug)]
pub struct Variant {
    pub name: String,
    pub ty: Option<String>,
    pub type_data: Option<ObjectData>
}
