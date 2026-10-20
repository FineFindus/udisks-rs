use std::ffi::CString;
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct UDisks2Path(String);
//https://github.com/search?q=zbus%3A%3AResult%3CVec%3C+language%3ARust&type=code&p=5

impl std::ops::Deref for UDisks2Path {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<OwnedValue> for UDisks2Path {
    type Error = <Vec<u8> as TryFrom<OwnedValue>>::Error;

    fn try_from(v: OwnedValue) -> Result<Self, Self::Error> {
        let data: Vec<u8> = v.try_into()?;

        Ok(Self(
            CString::from_vec_with_nul(data)
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
        ))
    }
}

impl TryFrom<Value<'_>> for UDisks2Path {
    type Error = <Vec<u8> as TryFrom<OwnedValue>>::Error;

    fn try_from(v: zbus::zvariant::Value<'_>) -> Result<Self, Self::Error> {
        let data: Vec<u8> = v.try_into()?;

        Ok(Self(
            CString::from_vec_with_nul(data)
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
        ))
    }
}
