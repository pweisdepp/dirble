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

use mockito::{server_address, mock, server_url};
use curl::easy::Easy2;
use crate::request::*;
use crate::arg_parse::GlobalOpts;
use crate::arg_parse::HttpVerb;
use crate::arg_parse::ScanOpts;
use std::sync::Arc;

#[test]
fn test_collector_write_and_clear() {

    let bytes:Vec<u8> = vec![4, 2, 7, 1];
    let mut collector = Collector {
        contents: Vec::new(),
        content_len: 0
    };

    collector.write(&bytes);
    assert_eq!(collector.contents, bytes);

    collector.clear_buffer();

    let mut vector: Vec<u8> = Vec::new();
    assert_eq!(collector.contents, vector);
}

#[test]
fn test_basic_request() {

    // get url of dummy http server
    let mut url: String = mockito::server_url().clone();

    // append folder to url
    url.push_str("/test");

    // create mock server
    let _m = mock("GET", "/test").create();

    let mut options = create_globalopts();
    let options = Arc::new(options);
    let mut easy = generate_easy(&options);

    let req = make_request(&mut easy, url);

    assert_eq!(req.code, 200);

}

#[test]
fn test_fabricate_response() {

    let request = RequestResponse {
        url: String::from("http://www.example.com/"),
        code: 0,
        content_len: 0,
        is_directory: false,
        is_listable: false,
        redirect_url: String::from(""),
        found_from_listable: true,
        parent_depth: 0
    };

    assert_eq!(request, fabricate_request_response(String::from("http://www.example.com/"), false, false));

}


fn generate_empty_easy() -> Easy2<Collector> {

    let easy = Easy2::new(Collector{contents: Vec::new(), content_len: 0});

    easy
}

fn create_globalopts() -> GlobalOpts {
    GlobalOpts {
        hostnames: Vec::new(),
        wordlist_files: Vec::new(),
        prefixes: Vec::new(),
        extensions: Vec::new(),
        max_threads: 1,
        proxy_enabled: false,
        proxy_address: String::from(""),
        proxy_auth_enabled: false,
        ignore_cert: false,
        show_htaccess: false,
        throttle: 0,
        max_recursion_depth: Some(2),
        user_agent: None,
        username: None,
        password: None,
        output_file: None,
        json_file: None,
        xml_file: None,
        verbose: false,
        silent: false,
        timeout: 5,
        max_errors: 5,
        wordlist_split: 3,
        scan_listable: false,
        cookies: None,
        headers: None,
        scrape_listable: false,
        whitelist: false,
        code_list: Vec::new(),
        is_terminal: false,
        no_color: false,
        disable_validator: false,
        http_verb: HttpVerb::Get,
        scan_opts: ScanOpts { scan_401: false, scan_403: false }
    }
}
