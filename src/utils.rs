use std::{fmt::Display, str::FromStr};

use serde::{de::Visitor, Deserialize, Deserializer};

/// See https://docs.rs/serde-aux/latest/serde_aux/field_attributes/fn.deserialize_number_from_string.html
pub fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
        StringOrInt::Number(i) => Ok(i),
    }
}

pub struct NonemptyStringVisitor;

impl<'de> Visitor<'de> for NonemptyStringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a non-empty string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let v = v.trim();
        if v.is_empty() {
            return Err(E::custom("Received empty string"));
        }

        Ok(v.to_string())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let v = v.trim();
        if v.is_empty() {
            return Err(E::custom("Received empty string"));
        }

        Ok(v.to_string())
    }
}
