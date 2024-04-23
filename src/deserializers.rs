use std::{fmt, str::FromStr};

use serde::{
    de::{self, DeserializeOwned, Error, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::models::Resolution;

pub(crate) fn option_number_from_string<'de, D>(
    deserializer: D,
) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    match serde_aux::prelude::deserialize_option_number_from_string::<i64, D>(
        deserializer,
    ) {
        Ok(r) => Ok(r),
        Err(_) => Ok(None),
    }
}

pub(crate) fn option_string_from_number<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(serde_aux::prelude::deserialize_string_from_number(
        deserializer,
    )?))
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

// pub fn vec_from_comma_separated_or_list<'de, D, T>(
//     deserializer: D,
// ) -> Result<Option<Vec<T>>, D::Error>
// where
//     D: Deserializer<'de>,
//     T: Deserialize<'de> + FromStr,
//     T::Err: std::fmt::Display,
// {
// }

pub fn vec_from_comma_separated_or_list<'de, D, T>(
    deserializer: D,
) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + FromStr,
    T::Err: fmt::Display,
{
    struct StringOrVecVisitor<T> {
        marker: std::marker::PhantomData<T>,
    }

    impl<'de, T> Visitor<'de> for StringOrVecVisitor<T>
    where
        T: Deserialize<'de> + FromStr,
        T::Err: fmt::Display,
    {
        type Value = Option<Vec<T>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value.is_empty() {
                Ok(None)
            } else {
                let parsed_list = value
                    .split(',')
                    .map(|s| s.trim().parse::<T>().map_err(de::Error::custom))
                    .collect::<Result<Vec<T>, E>>();
                parsed_list.map(Some)
            }
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = seq.next_element()? {
                values.push(value);
            }
            Ok(if values.is_empty() {
                None
            } else {
                Some(values)
            })
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

    deserializer.deserialize_any(StringOrVecVisitor {
        marker: std::marker::PhantomData,
    })
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
