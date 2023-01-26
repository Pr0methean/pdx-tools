use crate::{bytes::get_u32, AsarError, AsarHeader};

pub struct AsarArchive<'a> {
    header: AsarHeader,
    data: &'a [u8],
}

impl<'a> AsarArchive<'a> {
    pub fn try_parse(input: &'a [u8]) -> Result<Self, AsarError> {
        let rem = input;
        let (magic, rem) = get_u32(rem)?;
        let (_prelude_size, rem) = get_u32(rem)?;
        let (header_size, rem) = get_u32(rem)?;
        let (json_size, rem) = get_u32(rem)?;

        if magic != 4 {
            return Err(AsarError::AsarHeader);
        }

        let json_data = rem.get(..json_size as usize).ok_or(AsarError::Eof)?;
        let header: AsarHeader = serde_json::from_slice(json_data)?;
        let data = input
            .get(header_size as usize + 12..)
            .ok_or(AsarError::Eof)?;

        Ok(AsarArchive { header, data })
    }

    pub fn file_data(&'a self, file: &str) -> Option<&'a [u8]> {
        let file = self
            .header
            .files
            .iter()
            .find_map(|(name, x)| (name == file).then_some(x))?;
        file.data(self)
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    pub fn header(&self) -> &AsarHeader {
        &self.header
    }
}
