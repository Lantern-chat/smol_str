use super::*;

use schemars::{schema::Schema, JsonSchema};

impl JsonSchema for SmolStr {
    fn schema_name() -> String {
        String::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        String::json_schema(gen)
    }
}
