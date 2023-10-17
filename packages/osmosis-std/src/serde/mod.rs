pub mod as_str {
    use serde::{de, Deserialize, Deserializer, Serializer};
    use std::{fmt::Display, str::FromStr};

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        T::from_str(&s).map_err(de::Error::custom)
    }

    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Display,
    {
        serializer.serialize_str(&value.to_string())
    }
}

pub mod as_str_vec {
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
    use std::{fmt::Display, str::FromStr};

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        let vec_of_strings: Vec<String> = Vec::deserialize(deserializer)?;
        vec_of_strings
            .into_iter()
            .map(|s| T::from_str(&s).map_err(de::Error::custom))
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
        Binary(values.to_vec()).to_base64().serialize(serializer)
    }
}
