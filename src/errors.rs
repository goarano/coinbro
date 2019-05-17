use crate::cryptowatch;

error_chain! {
    links {
        Cryptowatch(cryptowatch::errors::Error, cryptowatch::errors::ErrorKind);
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
    }
}
