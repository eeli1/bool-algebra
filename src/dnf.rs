use crate::{validate_tabel, Token};

pub fn to_dnf(table: Vec<bool>, names: Vec<String>) -> Result<Vec<Token>, String> {
    validate_tabel(table, names)?;
    Ok(Vec::new())
}
