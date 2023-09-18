fn build_full_name(last_name: &str, first_name: &str) -> Result<String, String> {
    if last_name.is_empty() || first_name.is_empty() {
        return Err("空文字です".to_string());
    }

    let full_name = format!("{} {}", last_name, first_name);
    Ok(full_name)
}

fn main() -> Result<(), String> {
    let last_name = "yamada";
    let first_name = "";

    let full_name = build_full_name(last_name, first_name)?;

    println!("Full Name: {}", full_name);

    Ok(())
}

fn excute() -> Result<String, String> {
    let last_name = "yamada";
    let first_name = "";

    let full_name = build_full_name(last_name, first_name)?;

    println!("Full Name: {}", full_name);

    Ok(full_name)
}
