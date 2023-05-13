impl  DeBin for Foo {
    fn de_bin(o:&mut usize, d:&[u8]) -> core::result::Result<Self, nanoserde::DeBinErr> {
        let id: u16 = DeBin::de_bin(o,d)?;
        Ok(match id {
            0u16 => Self::A,
            1u16 => Self::B (DeBin::de_bin(o, d)?,),
            2u16 => Self::C {},
            _ => return core::result::Result::Err(nanoserde::DeBinErr{o:*o, l:0, s:d.len()})
        })
    }
}