use super::*;

use serde::de::IntoDeserializer;

// This code has been **heavily** inspired by the code written in the [serde](serde) crate

pub struct MessyJsonRawEnumDeserializer<'a> {
    pub variant: Cow<'a, str>,
    pub value: Option<MessyJsonValueRaw<'a>>,
}

pub struct MessyJsonRawVariantDeserializer<'a> {
    value: Option<MessyJsonValueRaw<'a>>,
}

impl<'de> serde::de::EnumAccess<'de> for MessyJsonRawEnumDeserializer<'de> {
    type Error = serde::de::value::Error;
    type Variant = MessyJsonRawVariantDeserializer<'de>;

    fn variant_seed<V>(
        self,
        seed: V,
    ) -> Result<(V::Value, MessyJsonRawVariantDeserializer<'de>), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = self.variant.into_deserializer();
        let visitor = MessyJsonRawVariantDeserializer { value: self.value };
        seed.deserialize(variant).map(|v| (v, visitor))
    }
}

impl<'de> serde::de::VariantAccess<'de> for MessyJsonRawVariantDeserializer<'de> {
    type Error = serde::de::value::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.value {
            Some(value) => serde::Deserialize::deserialize(value),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(value),
            None => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(MessyJsonValueRaw::Array(v)) => {
                serde::Deserializer::deserialize_any(MessyJsonRawSeqDeserializer::new(v), visitor)
            }
            Some(other) => Err(serde::de::Error::invalid_type(
                other.into(),
                &"tuple variant",
            )),
            None => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::UnitVariant,
                &"tuple variant",
            )),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(MessyJsonValueRaw::Obj(v)) => {
                serde::Deserializer::deserialize_any(MessyJsonRawMapDeserializer::new(v), visitor)
            }
            Some(other) => Err(serde::de::Error::invalid_type(
                other.into(),
                &"struct variant",
            )),
            None => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::UnitVariant,
                &"struct variant",
            )),
        }
    }
}
