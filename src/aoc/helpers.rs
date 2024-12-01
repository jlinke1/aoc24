pub fn open_inputs(path: &str) -> anyhow::Result<String> {
    let file = std::fs::File::open(path)?;
    let result = std::io::read_to_string(file)?;

    Ok(result)
}
