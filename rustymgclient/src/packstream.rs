use crate::value::MgValue;

pub fn decode(buf: &[u8]) -> Result<MgValue, String> {
    let marker = buf[0];
    match marker {
        0xC0 => Ok(MgValue::Null),
        0xC3 => Ok(MgValue::Bool(true)),
        0xC2 => Ok(MgValue::Bool(false)),
        0xC8 => Ok(MgValue::Int(buf[1] as i8 as i64)),
        _ => Err(format!("Unknown marker: {:#X}", marker)),
    }
}