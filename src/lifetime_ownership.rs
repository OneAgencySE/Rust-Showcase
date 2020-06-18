use serde::{Deserialize, Serialize};

/// Subjects: Ownership, Lifetime
/// we also started using an external crate named Serde for Serialize/Deserialize

/// A simple Data struct
///
/// We're creating a string literal and a String.
/// The difference is that one is put on the heap (String)
/// and one on the stack.
///
/// Many things on the stack needs a lifetime specifier.
/// It's done through the 'a where the A is just the name.
/// Here is says: Data has a lifetime, and the field name lives
/// for as long as the Data struct lives.
///
/// the Derive keyword here is a way to use "declarative macros"
/// The compiler will take the macros and generate implementations for us
/// Here we add serialization and de-serialization functionality to our Data struct.
/// we also add Debug which let's is print the value nicely and the Eq/partialEq let's us compare
/// using ==
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Data<'a> {
    name: &'a str, // Stack -> 'a to specify who long it needs to live
    email: String, // Heap -> Owned
}

/// Introducing the specific case of needing lifetime
///
/// Note: This is how we generate and write documentation in Rust
/// we can run cargo doc --open to generate docs and open them.
/// As you'll see we can write code in here that acts as a test using ```rust
/// We can even write pseudo code using ``rust ignore
///
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
/// ```rust ignore
/// // fn main() {
/// //     let short = "Shortest string".to_string();
/// //     let res = {
/// //         let long = String::from("Longest string for sure");
/// //         showcase::longest(&short, &long)
/// //     };
/// //     assert_eq!(res, &long);
/// // }
/// ```
///
/// We take 2 references and returns the longest.
/// Here we'd get "missing lifetime specifier" without this specification.
/// We have specified that a and b lives for equally as long here.
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

    // This is an example where longest can be called.
    // both input values lives within the same scope
    #[test]
    fn longest_works() {
        let short = "Shortest string".to_string();
        let long = String::from("Longest string for sure");
        let res = longest(&short, &long);
        assert_eq!(res, &long);
    }

    // here we get the error: borrowed value does not live long enough
    // This is because long will be deleted after we exit scope
    // #[test]
    // fn longest_nope() {
    //     let short = "Shortest string".to_string();
    //     let res = {
    //         let long = String::from("Longest string for sure");
    //         longest(&short, &long)
    //     };
    //     assert_eq!(res, &short);
    // }

    // Each time you leave a scope Rust will clean up that memory.
    // here we are moving the 'v' into a new scope and when it goes away
    // and the value is removed out of mem
    // #[test]
    // fn ownership_value_is_moved() {
    //     let mut v = Vec::new();
    //     v.push(5);
    //     let scope = { v };
    //     println!("{:?}", v);
    // }

    // Here we are using the '&' which means that we are 'lending' the value
    // instead of giving it away. This means that we'll still have the value going further
    #[test]
    fn ownership_borrow() {
        let mut v = Vec::new();
        v.push(5);

        let _scope = { &v };
        println!("{:?}", v);
    }

    // Here we simply use the serialize trait that we added to the Data struct using the drive keyword
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
