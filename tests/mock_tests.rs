
mod tests {
    use mockito::{server_address, mock};
    use std::process::Command;
    use assert_cmd::prelude::*;

    #[test]
    fn mockito_example() {
        #[cfg(test)]
        let url = &mockito::server_url();

        let _m = mock("GET", "/hello")
            .with_status(201)
            .create();

        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg(&url);
        cmd.assert().success();

        _m.assert();
    }

}