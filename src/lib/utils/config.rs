use std::{env, ffi::OsString};

/// Parse the port from either the provided command line argument, or the
/// `API_PORT` environment variable. If neither are set, use 3001.
pub fn parse_port(port: Option<OsString>) -> String {
    // get the port from the environment, or use 3001 if it's not set
    match port {
        // if the port is specified as a command line argument, use that
        Some(v) => v.into_string().expect("invalid port"),
        // otherwise, try to get the port from the environment
        _ => match env::var("API_PORT") {
            // if the port is set in the environment, use that
            Ok(v) => v,
            // otherwise, use 3001
            _ => "3001".into(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_port() {
        let cases = vec![(Some(OsString::from("3000")), "3000"), (None, "3001")];

        for case in cases {
            let port = parse_port(case.0);
            assert_eq!(port, case.1);
        }

        env::set_var("API_PORT", "3002");
        let port = parse_port(None);
        assert_eq!(port, "3002");
    }
}
