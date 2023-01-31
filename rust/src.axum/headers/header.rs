use axum::{extract::TypedHeader, headers::Header};

/// Generate String-focused, generic, custom typed [`Header`]'s.
#[allow(unused)]
macro_rules! header {
    ($tname:ident, $hname:ident, $sname:expr) => {
        static $hname: once_cell::sync::Lazy<axum::headers::HeaderName> =
            once_cell::sync::Lazy::new(|| axum::headers::HeaderName::from_static($sname));

        #[doc = "Generated custom [`axum::headers::Header`] for "]
        #[doc = $sname]
        #[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub(crate) struct $tname(pub(crate) String);

        impl std::fmt::Display for $tname {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::convert::From<&str> for $tname {
            fn from(item: &str) -> Self {
                $tname(item.to_string())
            }
        }

        impl axum::headers::Header for $tname {
            fn name() -> &'static axum::headers::HeaderName {
                &$hname
            }

            fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
            where
                I: Iterator<Item = &'i axum::headers::HeaderValue>,
            {
                values
                    .next()
                    .and_then(|v| v.to_str().ok())
                    .map(|x| $tname(x.to_string()))
                    .ok_or_else(axum::headers::Error::invalid)
            }

            fn encode<E>(&self, values: &mut E)
            where
                E: Extend<axum::headers::HeaderValue>,
            {
                if let Ok(value) = axum::headers::HeaderValue::from_str(&self.0) {
                    values.extend(std::iter::once(value));
                }
            }
        }
    };
}

/// Trait for returning header value directly for passing
/// along to client calls.
pub(crate) trait HeaderValue {
    fn header_value(&self) -> String;
}

impl<T> HeaderValue for TypedHeader<T>
where
    T: Header + std::fmt::Display,
{
    fn header_value(&self) -> String {
        self.0.to_string()
    }
}

impl<T> HeaderValue for &TypedHeader<T>
where
    T: Header + std::fmt::Display,
{
    fn header_value(&self) -> String {
        self.0.to_string()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use axum::{
        headers::{Header, HeaderMapExt},
        http,
    };

    header!(XDummyId, XDUMMY_ID, "x-dummy-id");

    fn test_decode<T: Header>(values: &[&str]) -> Option<T> {
        let mut map = http::HeaderMap::new();
        for val in values {
            map.append(T::name(), val.parse().unwrap());
        }
        map.typed_get()
    }

    fn test_encode<T: Header>(header: T) -> http::HeaderMap {
        let mut map = http::HeaderMap::new();
        map.typed_insert(header);
        map
    }

    #[test]
    fn test_dummy_header() {
        let s = "18312349-3139-498C-84B6-87326BF1F2A7";
        let dummy_id = test_decode::<XDummyId>(&[s]).unwrap();
        let headers = test_encode(dummy_id);
        assert_eq!(headers["x-dummy-id"], s);
    }
}
