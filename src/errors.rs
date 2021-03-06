use crate::cryptowatch;

error_chain! {
    links {
        Cryptowatch(cryptowatch::errors::Error, cryptowatch::errors::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        SerdeError(serde_json::error::Error);
    }

    errors {
        ExchangeNotFound(exchange: String) {
            description("exchange not found"),
            display("exchange not found: {}", exchange),
        }

        PairNotFound(pair: String) {
            description("pair not found"),
            display("pair not found: {}", pair),
        }

        ConfigFileNotFound(file: String) {
            description("config file not found"),
            display("config file not found: {}", file)
        }
    }
}
