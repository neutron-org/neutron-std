use serde::{
    de::{
        self,
        value::{I64Deserializer, U64Deserializer},
    },
    Deserialize,
};
use std::{
    fmt::{Display, Formatter},
    marker::PhantomData,
    str::FromStr,
};

struct StringOrNumberVisitor<T> {
    p: PhantomData<T>,
}
// The Visitor helps deserialize a number, both from its numerical JSON representation and from a string.
// For example, for the struct:
// ```rust
// struct Foo {
//     pub bar: i32;
// }
// ```
// Both JSON representations `'{"bar":"12"}'` and `'{"bar":12}'` will give the same result upon deserialization:
impl<'de, T> de::Visitor<'de> for StringOrNumberVisitor<T>
where
    T: Deserialize<'de>,
    T::Err: Display,
    T: FromStr,
{
    type Value = T;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a string or a number")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        T::deserialize(U64Deserializer::new(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        T::deserialize(I64Deserializer::new(value))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse::<T>().map_err(de::Error::custom)
    }
}

// Helps (de)serialize any type that implements FromStr trait as str
// For example some structs in Cosmos SDK returns ints as strs and this mod helps deal with that
pub mod as_str {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::{fmt::Display, marker::PhantomData, str::FromStr};

    use super::StringOrNumberVisitor;

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StringOrNumberVisitor { p: PhantomData })
    }

    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Display,
    {
        serializer.serialize_str(&value.to_string())
    }
}

// Helps (de)serialize any Option<T> that FromStr trait as str
// For example some structs in Cosmos SDK returns ints as strs and this mod helps deal with that
pub mod option_as_str {
    use serde::{de, Deserialize, Deserializer, Serializer};
    use std::{fmt::Display, str::FromStr};

    use super::NumberOrString;

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: FromStr + Deserialize<'de>,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        let encoded_string: Option<NumberOrString<T>> = Option::deserialize(deserializer)?;
        match encoded_string {
            Some(NumberOrString::String(s)) => T::from_str(&s).map(Some).map_err(de::Error::custom),
            Some(NumberOrString::Number(n)) => Ok(Some(n)),
            None => Ok(None),
        }
    }

    pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Display,
    {
        match value {
            None => serializer.serialize_none(),
            Some(v) => serializer.serialize_str(&v.to_string()),
        }
    }
}

pub mod as_str_vec {
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
    use std::{fmt::Display, str::FromStr};

    use super::NumberOrString;

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        T: FromStr + Deserialize<'de>,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        let vec_of_strings: Vec<NumberOrString<T>> = Vec::deserialize(deserializer)?;
        vec_of_strings
            .into_iter()
            .map(|v| match v {
                NumberOrString::Number(n) => Ok(n),
                NumberOrString::String(s) => T::from_str(&s).map_err(de::Error::custom),
            })
            .collect()
    }

    pub fn serialize<S, T>(values: &[T], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Display,
    {
        let vec_of_strings: Vec<String> = values.iter().map(|value| value.to_string()).collect();
        vec_of_strings.serialize(serializer)
    }
}

pub mod as_base64_encoded_string {
    use cosmwasm_std::Binary;
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded_string = String::deserialize(deserializer)?;
        Binary::from_base64(&encoded_string)
            .map(|b| b.to_vec())
            .map_err(de::Error::custom)
    }

    pub fn serialize<S>(values: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Binary::new(values.to_vec())
            .to_base64()
            .serialize(serializer)
    }
}

pub mod as_option_base64_encoded_string {
    use cosmwasm_std::Binary;
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded_string: Option<String> = Option::deserialize(deserializer)?;
        match encoded_string {
            Some(s) => Binary::from_base64(&s)
                .map(|b| Some(b.to_vec()))
                .map_err(de::Error::custom),
            None => Ok(None),
        }
    }

    pub fn serialize<S>(value: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(vec) => {
                let encoded_string = Binary::new(vec.clone()).to_base64();
                encoded_string.serialize(serializer)
            }
            None => serializer.serialize_none(),
        }
    }
}

// NumberOrString is a helper enum that helps us determine which
// JSON numeric representation we are working with. If it's a string,
// we will get the value `NumberOrString::String("-11")`.
// If we are dealing with a numeric representation,
// we will get `NumberOrString::Number(-11i64)`.
// Then, using pattern matching, we can select the appropriate algorithm to work with the data.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum NumberOrString<T> {
    Number(T),
    String(String),
}

#[test]
fn parse_test() {
    let int = "-11";
    let res = serde_json_wasm::from_str::<NumberOrString<i64>>(int).unwrap();
    assert_eq!(res, NumberOrString::Number(-11i64));

    let str = "\"-11\"";
    let res = serde_json_wasm::from_str::<NumberOrString<i64>>(str).unwrap();
    assert_eq!(res, NumberOrString::String("-11".to_string()));
}
