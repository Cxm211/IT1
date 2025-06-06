pub enum PrimitiveType {
    Long,
}

pub enum DataType {
    Primitive(PrimitiveType),
}

impl DataType {
    pub const LONG: Self = DataType::Primitive(PrimitiveType::Long);
}