// This file is part of Dirble - https://www.github.com/nccgroup/dirble
// Copyright (C) 2019 Izzy Whistlecroft <Izzy(dot)Whistlecroft(at)nccgroup(dot)com>
// Released as open source by NCC Group Plc - https://www.nccgroup.com/
//
// Dirble is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Dirble is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Dirble.  If not, see <https://www.gnu.org/licenses/>.

static OUTPUT_WITHOUT_ARGS: &'static str = "error: The following required arguments were not provided:
    <host|--host-file <host_file>...|--host <host_uri>...>
";

static OUTPUT_WITHOUT_HTTP: &'static str = "error: Invalid value for '<host_uri>': The provided target URI must start with http:// or https://\n";

#[cfg(test)]
mod arg_tests {
    use std::process::Command;

    #[test]
    fn call_without_host() {

        use crate::OUTPUT_WITHOUT_ARGS;

        let output = Command::new("./target/debug/dirble")
            .output()
            .expect("Executing dirble failed");

        let dirble_error = String::from_utf8_lossy(&output.stderr);

        let mut error_lines = dirble_error.lines();

        let mut dirble_output = String::new();

        if let Some(line) = error_lines.next() {
            dirble_output.push_str(line);
            dirble_output.push_str("\n");
        }

        if let Some(line) = error_lines.next() {
            dirble_output.push_str(line);
            dirble_output.push_str("\n");
        }

        assert_eq!(OUTPUT_WITHOUT_ARGS, dirble_output);
    }

    #[test]
    fn call_with_incorrect_url() {

        use crate::OUTPUT_WITHOUT_HTTP;

        let output = Command::new("./target/debug/dirble")
            .arg("www.abc.com")
            .output()
            .expect("Executing dirble failed");

        let dirble_error = String::from_utf8_lossy(&output.stderr);

        assert_eq!(OUTPUT_WITHOUT_HTTP, dirble_error);
    }

}
