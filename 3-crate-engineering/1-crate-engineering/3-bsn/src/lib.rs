use serde::{de::Visitor, Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug)]
/// Error creating BSN
// TODO: update the enum to make it more descriptive
// as there can be several reasons for a BSN to not be valid
pub enum Error {
    /// The BSN was invalid
    InvalidBsn,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidBsn => write!(f, "Invalid BSN number"),
        }
    }
}

/// A valid BSN (burgerservicenummer), a Dutch
/// personal identification number that is similar
/// to the US Social Security Number.
/// More info (Dutch): https://www.rvig.nl/bsn
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Bsn {
    inner: String,
}

impl Bsn {
    /// Try to create a new BSN. Returns `Err` if the passed string
    /// does not represent a valid BSN
    pub fn try_from_string<B: ToString>(bsn: B) -> Result<Self, Error> {
        let bsn_string = bsn.to_string();

        if Self::validate(bsn_string.as_str()).is_err() {
            return Err(Error::InvalidBsn);
        }

        Ok(Self {
            inner: bsn_string,
        })
    }

    /// Check whether the passed string represents a valid BSN.
    //  Returns `Err` if the passed string does not represent a valid BSN
    pub fn validate(bsn: &str) -> Result<(), Error> {
        let bsn_string_len = bsn.len();

        if bsn_string_len != 8 && bsn_string_len != 9 {
            Err(Error::InvalidBsn)?;
        }

        let mut result: i32 = 0;
        let mut multiplier: i32 = 9;

        for char in bsn.chars() {
            result += ((char as i32) - '0' as i32) * multiplier;
            multiplier -= 1;
            if multiplier == 1 {
                multiplier = -1;
            }
        }

        if result % 11 != 0 {
            Err(Error::InvalidBsn)?;
        }

        Ok(())
    }
}

impl Serialize for Bsn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.inner.as_str())
    }
}

impl<'de> Deserialize<'de> for Bsn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// A visitor for deserializing strings into `Bns`
        struct BsnVisitor;

        impl<'d> Visitor<'d> for BsnVisitor {
            type Value = Bsn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "A string representing a valid BSN")
            }

            fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error
            {
                self.visit_string(String::from(str))
            }


            fn visit_string<E>(self, str: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let str_len = str.len();

                if Bsn::validate(str.as_str()).is_err() {
                    return Err(serde::de::Error::invalid_length(str_len, &self));
                }

                Ok(Bsn {
                    inner: String::from(str)
                })
            }
        }

        Ok(deserializer.deserialize_any(BsnVisitor {})?)
    }
}

#[cfg(test)]
mod tests {
    use crate::Bsn;

    #[test]
    fn test_validation() {
        let bsns = include_str!("../valid_bsns.in").lines();
        bsns.for_each(|bsn| assert!(Bsn::validate(bsn).is_ok(), "BSN {bsn} is valid, but did not pass validation"));

        let bsns = include_str!("../invalid_bsns.in").lines();
        bsns.for_each(|bsn| assert!(Bsn::validate(bsn).is_err(), "BSN {bsn} invalid, but passed validation"));
    }

    #[test]
    fn test_serde() {
        let json = serde_json::to_string(&Bsn::try_from_string("999998456").unwrap()).unwrap();
        assert_eq!(json, "\"999998456\"");
        let bsn: Bsn = serde_json::from_str("\"999998456\"").unwrap();
        assert_eq!(bsn, Bsn::try_from_string("999998456".to_string()).unwrap());

        serde_json::from_str::<Bsn>("\"1112223333\"").unwrap_err();
    }
}
