use bincode::{Decode, Encode};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display, Formatter};
use std::io::Write;
use xml::writer::XmlEvent;

use yaserde::ser::Serializer as YaSerializer;
use yaserde::YaSerialize;
use yaserde_derive::YaDeserialize;

/// A special boolean struct designed for XML serialization, catering to the annoying requirement
/// of an Android mobile client that expects boolean values to be represented as "0" or "1" instead
/// of "true" or "false".
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    PartialOrd,
    YaDeserialize,
    Encode,
    Decode,
)]
pub struct SpecialBool {
    /// The inner boolean value.
    inner: bool,
}

impl SpecialBool {
    /// Creates a new `SpecialBool` with the given boolean value.
    pub fn new(inner: bool) -> Self {
        Self { inner }
    }
}

impl Display for SpecialBool {
    /// Formats the inner boolean as "1" for true and "0" for false.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", if self.inner { "1" } else { "0" })
    }
}

impl YaSerialize for SpecialBool {
    /// Serializes the boolean as "1" for true and "0" for false for XML content.
    fn serialize<W: Write>(
        &self,
        writer: &mut YaSerializer<W>,
    ) -> Result<(), String> {
        let content = self.to_string();
        let event = XmlEvent::characters(&content);
        writer.write(event).map_err(|e| e.to_string())
    }

    /// Does not modify attributes or namespace during serialization.
    fn serialize_attributes(
        &self,
        attributes: Vec<xml::attribute::OwnedAttribute>,
        namespace: xml::namespace::Namespace,
    ) -> Result<
        (
            Vec<xml::attribute::OwnedAttribute>,
            xml::namespace::Namespace,
        ),
        String,
    > {
        Ok((attributes, namespace))
    }
}

impl Serialize for SpecialBool {
    /// Serializes the boolean using Serde's boolean serialization mechanism.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(self.inner)
    }
}

impl<'de> Deserialize<'de> for SpecialBool {
    /// Experimental!
    /// Deserializes the boolean from a string or a boolean value.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner: bool =
            serde_aux::prelude::deserialize_bool_from_anything(deserializer)?;
        Ok(SpecialBool::new(inner))
    }
}

// impl<'de> Deserialize<'de> for SpecialBool {
//     /// Experimental!
//     /// Deserializes the boolean from a string or a boolean value.
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let cloneable_value: serde_json::Value =
//             Deserialize::deserialize(deserializer)?;

//         let maybe_string = String::deserialize(cloneable_value.clone());
//         match maybe_string {
//             Ok(string) => {
//                 let inner: bool = match string.as_str() {
//                     "1" | "true" | "True" | "TRUE" => true,
//                     "0" | "false" | "False" | "FALSE" => false,
//                     _ => {
//                         return Ok(SpecialBool::new(false));
//                     }
//                 };

//                 Ok(SpecialBool::new(inner))
//             }
//             Err(_) => {
//                 let maybe_bool = bool::deserialize(cloneable_value);
//                 match maybe_bool {
//                     Ok(bool) => Ok(SpecialBool::new(bool)),
//                     Err(_) => Ok(SpecialBool::new(false)),
//                 }
//             }
//         }
//     }
// }
