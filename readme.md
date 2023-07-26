# Schema yaml validation
Example project of parsing yaml files into Rust types via serde and validating the yaml documents in VSCode (requires the [yaml extension](https://github.com/redhat-developer/vscode-yaml)) by generating JSON schema to lint the YAML in the build.rs script.

## How does this work
1. Rust types are defined in the ./types crate. The types derive `serde::Deserialize` and `schemars::JsonSchema`.
2. The program `./program/build.rs` file generates a JSON Schema from the type into `./program/schemas`
3. The `.vscode/settings.json` file references the `program/data/example.yaml` data file and the schema to validate against
4. VSCode will now validate the `example.yaml` file automatically when edited
5. The program to load the yaml file into a Rust struct can be called via `cargo run program`

## Some possible improvements
- Generate the `.vscode/settings.json` config for 'yaml.schemas' inside the build.rs script
- Update `program/build.rs` to read Rust types generically from a directory (parse using `syn`) or referenced from the yaml file
- Create git hooks to validate the yaml based on the generated JSON Schema or Rust types
