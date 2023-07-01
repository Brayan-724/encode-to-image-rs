pub fn is_similar(source: u8, target: u8, threshold: u8) -> bool {
    let diff = source.abs_diff(target);
    diff < threshold
}

pub trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for u8 {
    fn to_hex(&self) -> String {
        format!("{self:0>2X?}")
    }
}

impl ToHex for u16 {
    fn to_hex(&self) -> String {
        format!("{self:0>4X?}")
    }
}

impl ToHex for u32 {
    fn to_hex(&self) -> String {
        format!("{self:0>8X?}")
    }
}
