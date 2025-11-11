pub mod chrono_human {
    use chrono::{DateTime, FixedOffset, Utc};
    use serde::{
        Deserialize, Serialize, Serializer,
        de::{Error, Visitor},
    };

    const TIME_FMT: &str = "%Y-%m-%d %H:%M:%S%z";

    pub struct CustomDateTime(DateTime<Utc>);

    impl Serialize for CustomDateTime {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let ts = self.0.format(TIME_FMT);
            serializer.collect_str(&ts)
        }
    }

    struct TimestampVisitor;

    impl<'de> Visitor<'de> for TimestampVisitor {
        type Value = CustomDateTime;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("timestamp")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let fixed =
                DateTime::<FixedOffset>::parse_from_str(v, TIME_FMT).map_err(Error::custom)?;
            Ok(CustomDateTime(fixed.to_utc()))
        }
    }

    impl<'de> Deserialize<'de> for CustomDateTime {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_str(TimestampVisitor)
        }
    }
}
