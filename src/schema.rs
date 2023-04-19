use schemars::JsonSchema;

use crate::ThinVec;

impl<T: JsonSchema> JsonSchema for ThinVec<T> {
    fn schema_name() -> String {
        <Vec<T> as JsonSchema>::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <Vec<T> as JsonSchema>::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        false
    }
}
