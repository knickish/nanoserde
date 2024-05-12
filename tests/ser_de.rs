#![cfg(any(feature = "binary", feature = "json", feature = "ron"))]
#[cfg(feature = "binary")]
use nanoserde::{DeBin, SerBin};
#[cfg(feature = "json")]
use nanoserde::{DeJson, SerJson};
#[cfg(feature = "ron")]
use nanoserde::{DeRon, SerRon};

#[cfg(feature = "no_std")]
use hashbrown::HashMap;

#[cfg(not(feature = "no_std"))]
use std::collections::HashMap;

#[test]
fn ser_de() {
    #[derive(PartialEq, Debug)]
    #[cfg_attr(feature = "binary", derive(DeBin, SerBin))]
    #[cfg_attr(feature = "json", derive(DeJson, SerJson))]
    #[cfg_attr(feature = "ron", derive(DeRon, SerRon))]
    pub struct Test {
        pub a: i32,
        pub b: f32,
        c: Option<String>,
        d: Option<String>,
        e: Option<HashMap<String, String>>,
        f: Option<([u32; 4], String)>,
        g: (),
    }

    let mut map = HashMap::new();
    map.insert("a".to_string(), "b".to_string());

    let test: Test = Test {
        a: 1,
        b: 2.,
        c: Some("asd".to_string()),
        d: None,
        e: Some(map),
        f: Some(([1, 2, 3, 4], "tuple".to_string())),
        g: (),
    };

    #[cfg(feature = "binary")]
    {
        let bytes = SerBin::serialize_bin(&test);
        let test_deserialized = DeBin::deserialize_bin(&bytes).unwrap();
        assert_eq!(test, test_deserialized);
    }

    #[cfg(feature = "json")]
    {
        let bytes = SerJson::serialize_json(&test);
        let test_deserialized = DeJson::deserialize_json(&bytes).unwrap();
        assert_eq!(test, test_deserialized);
    }

    #[cfg(feature = "ron")]
    {
        let bytes = SerRon::serialize_ron(&test);
        let test_deserialized = DeRon::deserialize_ron(&bytes).unwrap();
        assert_eq!(test, test_deserialized);
    }
}
