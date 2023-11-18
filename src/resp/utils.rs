pub fn to_debug_string(resp: &Vec<u8>) -> Result<String, std::string::FromUtf8Error> {
    let debug_string = String::from_utf8(resp.to_vec())?;

    let debug_string = debug_string.replace("\r", "\\r");
    let debug_string = debug_string.replace("\n", "\\n");

    Ok(debug_string)
}
