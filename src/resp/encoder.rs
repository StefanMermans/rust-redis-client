pub fn encode_string<T: AsRef<str>>(value: T) -> Vec<u8> {
    let str_ref = value.as_ref();

    return format!("${}\r\n{}\r\n", str_ref.len(), str_ref).into()
}

