use std::{fs::File, io::Read, num::ParseIntError};

/// This is a simple struct that will represent our Error
/// It contains a simple String and can be printed nicely
///
/// we'll use this one to show you the '?' operator that lets us
/// simplify error handling and let us use less matching!
///
/// We've also implemented the From trait,
/// it helps us go from std::io::Error into
/// the error that we want, eg MyError
///
/// It uses this From, but it also generates Into()
/// From:
/// ```rust
/// let i = std::io::Error::new(std::io::ErrorKind::Other, "oh no!");
/// let my_error_from: MyError::from(i);
/// ```
/// Into:
/// ```rust
/// let i = std::io::Error::new(std::io::ErrorKind::Other, "oh no!");
/// let my_error_into: MyError = i.into();
/// let my_error_from: MyError::from(i);
/// ```
#[derive(Debug)]
pub struct MyError {
    msg: String,
}

/// As mentioned here's the From implementation
// to help us go from std::io::Error into MyError
impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        MyError { msg: e.to_string() }
    }
}

/// Same thing as above but from ParseIntError to MyError
impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError { msg: e.to_string() }
    }
}

/// Scenario: We're reading a configuration file.
/// We outsource the reading of the file to a separate method that returns a std::io::Result<File>
/// We use the "if let" syntax to check if the value is Ok and then we read the file.
///
/// There are multiple areas where we could fail.
/// - Reading the file from disk
/// - Reading the file into memory as String
/// - Parsing the content of the file
///
/// These are all scenarios we need to handle. Either through match using
/// ```rust ignore
/// let file = read_conf_file("./num.txt");
/// match file {
///     Ok(file) => ... ,
///     Err(e) => ...
/// }
/// ```
/// Or through the if let syntax as we do here.
/// We could solve this through .map(...).or_else(). also.
/// Tip: Try to refactor this into match/.map
pub fn get_conf_val() -> Result<Setting, MyError> {
    let file = read_conf_file("./num.txt");

    if let Ok(mut f) = file {
        let mut content = String::new();
        // See the '?' operator in action, it tells the compiler to propagate the error upwards, and with the help of 'From'
        // we it converts the io error into MyError!
        f.read_to_string(&mut content)?;

        // I mean, we could just unwrap and Panic/die if the file content is bad
        // But '?' or a match/if let/map_error might be a better choice
        // tip: Change file content to something that's not an i32 and use ? here instead of unwrap
        let n = content.parse::<i32>().unwrap();
        Ok(Setting { n })
    } else {
        Ok(Setting { n: 5 })
    }
}

/// We're simply reading the file here, we could be checking stuff like:
/// - Is the file in correct format?
/// - Is the file locked?
fn read_conf_file(path: &str) -> std::io::Result<File> {
    File::open(path)
}

/// This is simply a carrier of settings data
pub struct Setting {
    n: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Here's a simple test to see if we get our result back.
    // here we could handle any Errors if we wanted to, but we just unwrap
    // we've already tested that it is ok.
    #[test]
    fn read_conf_42() {
        let res = get_conf_val();
        assert!(&res.is_ok());
        let e = res.unwrap();
        assert_eq!(e.n, 42);
    }
}
