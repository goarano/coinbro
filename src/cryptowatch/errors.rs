error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
        ReqwestError(reqwest::Error);
        SerdeError(serde_json::error::Error);
    }

    // Define additional `ErrorKind` variants.  Define custom responses with the
    // `description` and `display` calls.
    errors {
        ParseError(m: String) {
            description("parsing issue"),
            display("parsing issue: {}", m),
        }
    }
}
