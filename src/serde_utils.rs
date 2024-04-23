use crate::models::Resolution;
use serde::de::Error as DeError;
use serde::de::{self, DeserializeOwned, Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;
use tracing::error;

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

pub fn deserialize_comma_seperated_number<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<i32>>, D::Error>
where
    D: Deserializer<'de>,
{
    match Deserialize::deserialize(deserializer)? {
        Some::<String>(s) => {
            if s.is_empty() {
                return Ok(None);
            }
            let r: Vec<i32> =
                s.split(',').map(|s| s.parse().unwrap()).collect();
            Ok(Some(r))
        }
        None => Ok(None),
    }
}

pub fn deserialize_comma_separated_string<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    match Deserialize::deserialize(deserializer)? {
        Some::<String>(s) => {
            let r: Vec<String> = s.split(',').map(|s| s.to_owned()).collect();
            Ok(Some(r))
        }
        None => Ok(None),
    }
}

pub fn deserialize_screen_resolution<'de, D>(
    deserializer: D,
) -> Result<Vec<Resolution>, D::Error>
where
    D: Deserializer<'de>,
{
    match Deserialize::deserialize(deserializer)? {
        Some::<String>(s) => {
            let cleaned_string: String = s
                .chars()
                .filter(|c| c.is_numeric() || *c == 'x' || *c == ',')
                .collect();
            let r: Vec<Resolution> = cleaned_string
                .split(',')
                .map(|s| {
                    let k: Vec<i64> =
                        s.split('x').map(|s| s.parse().unwrap()).collect();
                    Resolution {
                        width: k[0],
                        height: k[1],
                    }
                })
                .collect();
            Ok(r)
        }
        None => Ok(vec![]),
    }
}

pub fn optional_int_from_str<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<i64>, D::Error> {
    struct IntOrStringVisitor;

    impl<'de> Visitor<'de> for IntOrStringVisitor {
        type Value = Option<i64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(
                "an integer or a string representation of an integer",
            )
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value))
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match value.parse::<i64>() {
                Ok(val) => Ok(Some(val)),
                Err(_) => Err(E::custom("failed to parse integer from string")),
            }
        }
    }

    deserializer.deserialize_option(IntOrStringVisitor)
}

// Saving just in case
//
// pub fn int_from_str<'de, D>(
//     deserializer: D,
// ) -> Result<Option<i64>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     match Deserialize::deserialize(deserializer)? {
//         Some::<i64>(s) => {
//             Ok(Some(s))
//         },
//         None => Ok(None),
//     }
// }

// pub fn int_from_str<'de, D: Deserializer<'de>>(de: D) -> Result<Option<i64>, D::Error> {
//     struct Visitor;

//     impl<'de> serde::de::Visitor<'de> for Visitor {
//         type Value = Option<i64>;

//         fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             f.write_str("a boolean")
//         }

//         fn visit_str<E: serde::de::Error>(self, val: &str) -> Result<Option<i64>, E> {
//            dbg!("yess");
//            Ok(Some(val.parse::<i64>().unwrap()))
//             // match val {
//             //     v if uncased::eq(v, "true") => Ok(true),
//             //     v if uncased::eq(v, "false") => Ok(false),
//             //     s => Err(E::invalid_value(Unexpected::Str(s), &"true or false"))
//             // }
//         }

//         // fn visit_u64<E: de::Error>(self, n: u64) -> Result<bool, E> {
//         //     match n {
//         //         0 | 1 => Ok(n != 0),
//         //         n => Err(E::invalid_value(Unexpected::Unsigned(n), &"0 or 1"))
//         //     }
//         // }

//         // fn visit_i64<E: de::Error>(self, n: i64) -> Result<bool, E> {
//         //     match n {
//         //         0 | 1 => Ok(n != 0),
//         //         n => Err(E::invalid_value(Unexpected::Signed(n), &"0 or 1"))
//         //     }
//         // }
//     }

//     de.deserialize_any(Visitor)
// }

// A visitor to handle deserialization from a number to an Option<String>
struct StringFromNumberVisitor;

impl<'de> de::Visitor<'de> for StringFromNumberVisitor {
    type Value = Option<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a number or null")
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NumberToStringVisitor)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(value.to_string()))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(value.to_string()))
    }

    // Implement other visit_* methods as needed

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub(crate) fn deserialize_option_string_from_number<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(StringFromNumberVisitor)
}

pub fn deserialize_option_number_from_string<'de, D>(
    deserializer: D,
) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(NumberOrStringVisitor)

    // deserializer.deserialize_any(NumberOrStringVisitor)
}

pub fn deserialize_host<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let host: Option<String> = Deserialize::deserialize(deserializer)?;
    Ok(host.map(|mut s| {
        if s.ends_with('/') {
            s.pop();
        }
        s
    }))
}

pub fn deserialize_comma_separated<'de, D, T>(
    deserializer: D,
) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned + FromStr,
    T::Err: std::fmt::Display,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;
    match s {
        Some(s) => s
            .split(',')
            .map(|s| s.trim().parse::<T>().map_err(Error::custom))
            .collect::<Result<Vec<T>, D::Error>>()
            .map(Some),
        None => Ok(None),
    }
}

struct NumberToStringVisitor;
impl<'de> Visitor<'de> for NumberToStringVisitor {
    type Value = Option<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a number or a string containing a number")
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NumberToStringVisitor)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(value.to_string()))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(value.to_string()))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(s.to_string()))
    }
}

struct NumberOrStringVisitor;
impl<'de> Visitor<'de> for NumberOrStringVisitor {
    type Value = Option<i64>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a number or a string containing a number")
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NumberFromStringVisitor)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Some(v))
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        s.parse::<i64>().map(Some).map_err(de::Error::custom)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

struct NumberFromStringVisitor;

impl<'de> Visitor<'de> for NumberFromStringVisitor {
    type Value = Option<i64>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string containing a number, a number, or null")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        match value.parse::<i64>() {
            Ok(num) => Ok(Some(num)),
            Err(e) => {
                error!("Failed to parse string '{}' as i64: {}", value, e);
                Err(E::custom(format!("could not parse string as i64: {}", e)))
            }
        }
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        Ok(Some(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        // Consider handling potential overflow if value > i64::MAX
        Ok(Some(value as i64))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        Ok(None)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        Ok(None)
    }
}
