use std::fs;
use example_types::Example;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //open the example yaml file against the type in the lib
    let contents = fs::read_to_string("program/data/example.yaml")?;
    let parsed: Example = serde_yaml::from_str(&contents)?;

    dbg!(parsed);

    Ok(())
}
