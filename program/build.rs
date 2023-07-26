use std::{fs::File, io::Write};
use example_types::Example;
use schemars::schema_for;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_for!(Example);
    let schema_str = serde_json::to_string_pretty(&schema).unwrap();

    let mut file = File::create("schemas/example.json")?;
    file.write_all(schema_str.as_bytes())?;

    Ok(())
}
