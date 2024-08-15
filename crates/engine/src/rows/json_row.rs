use schemajs_primitives::column::types::DataValue;
use schemajs_primitives::column::Column;
use schemajs_primitives::table::Table;
use schemajs_query::primitives::{Row, ShardKey};
use schemajs_query::serializer::RowSerializationError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Debug, Formatter};

#[derive(Serialize, Deserialize)]
pub struct RowData {
    pub table: String,
    pub value: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct RowJson {
    value: RowData,
}

impl From<RowData> for RowJson {
    fn from(value: RowData) -> Self {
        RowJson { value }
    }
}

impl schemajs_query::serializer::RowSerializer<RowJson> for RowJson {
    fn serialize(&self) -> Result<Vec<u8>, RowSerializationError> {
        serde_json::to_vec(&self.value).map_err(|e| {
            RowSerializationError::SerializationError("Error serializing row".to_string())
        })
    }

    fn deserialize(&self, data: &[u8]) -> Result<RowJson, RowSerializationError> {
        let data = serde_json::from_slice::<RowData>(data).map_err(|e| {
            RowSerializationError::DeserializationError("Error Deserializing row".to_string())
        })?;

        Ok(Self { value: data })
    }
}

impl Debug for RowJson {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<Vec<u8>> for RowJson {
    fn from(value: Vec<u8>) -> Self {
        todo!()
    }
}

impl ShardKey for RowJson {}

impl Row<RowJson> for RowJson {
    fn get_value(&self, column: &Column) -> Option<DataValue> {
        let potential_val = self.value.value.get(column.name.to_string());
        match potential_val {
            None => return None,
            Some(val) => Some(DataValue::from((column, val))),
        }
    }

    fn get_table_name(&self) -> String {
        self.value.table.clone()
    }

    fn validate(&self) -> bool {
        todo!()
    }
}