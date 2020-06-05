use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Data<'a> {
    name: &'a str, // Stack -> 'a to specify who long it needs to live
    email: String, // Heap -> Owned
}

/// Introducing the specifc case of needing lifetime
///
/// run 'cargo doc --all' to test this code:
/// ```rust
/// fn main() {
///     let short = "Shortest string".to_string();
///     let long = String::from("Longest string for sure");
///     let res = showcase::longest(&short, &long);
///     assert_eq!(res, &long);
/// }
/// ```
///
/// Recall what did not work due to lifetime restrictions:
/// ```rust
/// // fn main() {
/// //     let short = "Shortest string".to_string();
/// //     let res = {
/// //         let long = String::from("Longest string for sure");
/// //         showcase::longest(&short, &long)
/// //     };
/// //     assert_eq!(res, &long);
/// // }
/// ```
pub fn longest<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_data() {
        let input = Data {
            name: "Kalle",
            email: "kalle@balle.se".to_string(), // String::from("")
        };

        let result = serde_json::to_string(&input).unwrap();
        assert_eq!(
            result,
            "{\"name\":\"Kalle\",\"email\":\"kalle@balle.se\"}".to_string()
        );
    }

    #[test]
    fn deserialize_data() {
        let input = "{\"name\":\"Kalle\",\"email\":\"kalle@balle.se\"}".to_string();
        let expected = Data {
            name: "Kalle",
            email: "kalle@balle.se".to_string(), // String::from("")
        };

        let result: Data = serde_json::from_str(&input).unwrap();
        assert_eq!(result, expected);
    }
}
