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

use crate::request::RequestResponse;

#[test]
fn check_directory() {

    let request = RequestResponse {
        url: String::from("http://www.example.com/parent/directory/file.txt"),
        code: 0,
        content_len: 0,
        is_directory: false,
        is_listable: false,
        redirect_url: String::from(""),
        found_from_listable: true,
        parent_depth: 0
    };

    let directory = super::directory_name(&request);

    assert_eq!(directory, "http://www.example.com/parent/directory");

}

#[test]
fn check_directory_trailing_slash_removal() {

    let request = RequestResponse {
        url: String::from("http://www.example.com/parent/directory/"),
        code: 0,
        content_len: 0,
        is_directory: true,
        is_listable: false,
        redirect_url: String::from(""),
        found_from_listable: true,
        parent_depth: 0
    };

    let directory = super::directory_name(&request);

    assert_eq!(directory, "http://www.example.com/parent/directory");

}