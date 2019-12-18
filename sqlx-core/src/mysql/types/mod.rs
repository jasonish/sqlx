use super::protocol::{FieldType, ParameterFlag};
use crate::{
    mysql::MySql,
    types::{HasTypeMetadata, TypeMetadata},
};

pub mod binary;
pub mod boolean;
pub mod character;
pub mod numeric;

#[cfg(feature = "chrono")]
pub mod chrono;

#[derive(Debug)]
pub struct MySqlTypeMetadata {
    pub field_type: FieldType,
    pub param_flag: ParameterFlag,
}

impl HasTypeMetadata for MySql {
    type TypeMetadata = MySqlTypeMetadata;
    type TypeId = u8;
}

impl TypeMetadata<u8> for MySqlTypeMetadata {
    fn type_id_eq(&self, other: &u8) -> bool {
        &self.field_type.0 == other
    }
}
