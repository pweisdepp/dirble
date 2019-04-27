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

extern crate assert_cmd;

#[cfg(test)]
mod arg_tests {
    use std::process::Command;
    use assert_cmd::prelude::*;

    #[test]
    fn call_without_host() {

        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.assert().failure();
    }

    #[test]
    fn call_without_valid_http() {

        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("www.example.com");
        cmd.assert().failure();
    }

    #[test]
    fn call_with_negative_threads() {

        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("-t").arg("-1").arg("http://www.example.com");
        cmd.assert().failure();
    }

    #[test]
    fn call_with_float_threads() {

        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("-t").arg("0.5").arg("http://www.example.com");
        cmd.assert().failure();
    }

}