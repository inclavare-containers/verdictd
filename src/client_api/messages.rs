extern crate serde;

use self::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyProviderInput {
    op: String,
    pub keywrapparams: KeyWrapParams,
    pub keyunwrapparams: KeyUnwrapParams,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyWrapParams {
    pub ec: Option<Ec>,
    pub optsdata: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ec {
    pub Parameters: HashMap<String, Vec<String>>,
    pub DecryptConfig: Dc,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyWrapOutput {
    pub keywrapresults: KeyWrapResults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyWrapResults {
    pub annotation: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyUnwrapParams {
    pub dc: Option<Dc>,
    pub annotation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dc {
    pub Parameters: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyUnwrapOutput {
    pub keyunwrapresults: KeyUnwrapResults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyUnwrapResults {
    pub optsdata: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_provider_input_serde() {
        let input = KeyProviderInput {
            op: "wrapKey".to_owned(),
            keywrapparams: KeyWrapParams {
                ec: None,
                optsdata: Some("test".to_owned()),
            },
            keyunwrapparams: KeyUnwrapParams {
                dc: None,
                annotation: None,
            },
        };

        let json = serde_json::to_string(&input).unwrap();
        let expected_json = r#"{"op":"wrapKey","keywrapparams":{"ec":null,"optsdata":"test"},"keyunwrapparams":{"dc":null,"annotation":null}}"#;
        assert_eq!(json, expected_json);

        let deserialized: KeyProviderInput = serde_json::from_str(expected_json).unwrap();
        assert_eq!(deserialized.op, input.op);
        assert_eq!(
            deserialized.keywrapparams.optsdata.unwrap(),
            input.keywrapparams.optsdata.unwrap()
        );
    }

    #[test]
    fn test_key_wrap_output_serde() {
        let output = KeyWrapOutput {
            keywrapresults: KeyWrapResults {
                annotation: vec![1, 2, 3],
            },
        };

        let json = serde_json::to_string(&output).unwrap();
        let expected_json = r#"{"keywrapresults":{"annotation":[1,2,3]}}"#;
        assert_eq!(json, expected_json);

        let deserialized: KeyWrapOutput = serde_json::from_str(expected_json).unwrap();
        assert_eq!(
            deserialized.keywrapresults.annotation,
            output.keywrapresults.annotation
        );
    }

    #[test]
    fn test_key_unwrap_output_serde() {
        let output = KeyUnwrapOutput {
            keyunwrapresults: KeyUnwrapResults {
                optsdata: vec![4, 5, 6],
            },
        };

        let json = serde_json::to_string(&output).unwrap();
        let expected_json = r#"{"keyunwrapresults":{"optsdata":[4,5,6]}}"#;
        assert_eq!(json, expected_json);

        let deserialized: KeyUnwrapOutput = serde_json::from_str(expected_json).unwrap();
        assert_eq!(
            deserialized.keyunwrapresults.optsdata,
            output.keyunwrapresults.optsdata
        );
    }
}
