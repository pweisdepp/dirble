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

use mockito::{mock, Matcher};
use crate::request::*;
use crate::arg_parse::GlobalOpts;
use crate::arg_parse::HttpVerb;
use crate::arg_parse::ScanOpts;
use std::sync::Arc;
use std::string::String;
use std::clone::Clone;

#[test]
fn test_collector_write_and_clear() {

    let bytes:Vec<u8> = vec![4, 2, 7, 1];
    let mut collector = Collector {
        contents: Vec::new(),
        content_len: 0
    };
    match collector.write(&bytes) {
        Ok(_v) => {},
        Err(_e) => {
            println!("Error writing to collector buffer");
        }
    }

    assert_eq!(collector.contents, bytes);

    collector.clear_buffer();

    let vector: Vec<u8> = Vec::new();
    assert_eq!(collector.contents, vector);
}

#[test]
fn test_basic_request() {

    // get url of dummy http server
    let url: String = mockito::server_url().clone();

    // create mock server
    let m = mock("GET", Matcher::Any).create();

    let options = create_globalopts();
    let options = Arc::new(options);
    let mut easy = generate_easy(&options);

    let req = make_request(&mut easy, url);

    assert_eq!(req.code, 200);
    m.assert();
}

#[test]
fn test_wrong_url() {

    let url: String = mockito::server_url().clone() + "test";

    let _m1 = mock("GET", "/test/")
        .with_status(10)
        .create();

    let options = Arc::new(create_globalopts());
    let mut easy = generate_easy(&options);

    let result = make_request(&mut easy, url.clone());

    let mut request = fabricate_request_response(url, false, false);
    request.found_from_listable = false;

    assert_eq!(result, request);

}

#[test]
fn test_redirect_and_listable() {

    // get url of dummy http server
    let mut url: String = mockito::server_url().clone();

    // append folder to url
    url.push_str("/test");

    let new_url = url.clone() + "/";

    // create mock server with directory
    let m1 = mock("GET", "/test")
        .with_status(301)
        .with_header("Location", &new_url)
        .create();

    let m2 = mock("GET", "/test/")
        .with_status(200)
        .with_body("parent directory")
        .create();

    let options = Arc::new(create_globalopts());
    let mut easy = generate_easy(&options);

    let req = make_request(&mut easy, url.clone());

    assert_eq!(req.code, 301);
    m1.assert();

    let result = listable_check(&mut easy, url.clone(), Some(2), 0, true);

    let result = &result[0];

    let mut request = fabricate_request_response(url + "/", true, true);
    request.code = 200;
    request.found_from_listable = false;
    request.content_len = "parent directory".len();

    assert_eq!(result, &request);
    m2.assert();
}

#[test]
fn test_unlistable() {

    // get url of dummy http server
    let mut url: String = mockito::server_url().clone();

    // append folder to url
    url.push_str("/test");

    // create mock server with directory
    let m1 = mock("GET", "/test/")
        .with_status(200)
        .with_body("no match")
        .create();

    let options = Arc::new(create_globalopts());
    let mut easy = generate_easy(&options);

    let _req = make_request(&mut easy, url.clone());

    let result = listable_check(&mut easy, url.clone(), Some(2), 0, true);

    let result = &result[0];

    let mut request = fabricate_request_response(url + "/", true, false);
    request.code = 200;
    request.found_from_listable = false;
    request.content_len = "no match".len();

    assert_eq!(result, &request);
    m1.assert();
}

#[test]
fn test_folder() {

    // get url of dummy http server
    let mut url: String = mockito::server_url().clone();

    // append folder to url
    url.push_str("/test");

    // create mock server with directory
    let m1 = mock("GET", "/test/")
        .with_status(10)
        .with_body("no match")
        .create();

    let options = Arc::new(create_globalopts());
    let mut easy = generate_easy(&options);

    let result = listable_check(&mut easy, url.clone(), Some(2), 0, true);

    let result = &result[0];

    let mut request = fabricate_request_response(url + "/", true, false);
    request.code = 10;
    request.found_from_listable = false;
    request.content_len = "no match".len();

    assert_eq!(result, &request);
    m1.assert();
}

#[test]
fn test_scrapable_recursive() {

    let url: String = mockito::server_url().clone() + "/test/";

    let url_to_scrape1 = r#" parent directory <a href="http://127.0.0.1:1234/test/dir1/">"#;
    let url_to_scrape2 = r#" parent directory <a href="http://127.0.0.1:1234/test/dir1/dir2/">"#;
    let url_to_scrape3 = r#" parent directory <a href="http://127.0.0.1:1234/test/dir1/dir2/file">"#;


    let m1 = mock("GET", "/test/")
        .with_status(200)
        .with_body(&url_to_scrape1)
        .create();

    let m2 = mock("GET", "/test/dir1/")
        .with_status(200)
        .with_body(&url_to_scrape2)
        .create();

    let m3 = mock("GET", "/test/dir1/dir2/")
        .with_status(200)
        .with_body(url_to_scrape3)
        .create();

    let _m4 = mock("GET", "/test/dir1/dir2/file")
        .with_status(200)
        .create();

    let mut options = create_globalopts();
    options.scrape_listable = true;
    options.max_recursion_depth = None;

    let options = Arc::new(options);

    let mut easy = generate_easy(&options);

    let _result: Vec<RequestResponse> = listable_check(&mut easy, url.clone(), None, 0, true);

    m1.assert();
    m2.assert();
    m3.assert();

}

#[test]
fn test_scrapable_recursive_max_depth() {

    let url: String = mockito::server_url().clone() + "/test/";

    let url_to_scrape1 = r#" parent directory <a href="http://127.0.0.1:1234/test/dir1/">"#;
    let url_to_scrape2 = r#" parent directory <a href="http://127.0.0.1:1234/test/dir1/dir2/">"#;
    let url_to_scrape3 = r#" parent directory <a href="http://127.0.0.1:1234/test/dir1/dir2/file">"#;

    let m1 = mock("GET", "/test/")
        .with_status(200)
        .with_body(&url_to_scrape1)
        .create();

    let m2 = mock("GET", "/test/dir1/")
        .with_status(200)
        .with_body(&url_to_scrape2)
        .create();

    let _m3 = mock("GET", "/test/dir1/dir2/")
        .with_status(200)
        .with_body(url_to_scrape3)
        .create();

    let _m4 = mock("GET", "/test/dir1/dir2/file")
        .with_status(200)
        .create();

    let mut options = create_globalopts();
    options.scrape_listable = true;

    let options = Arc::new(options);

    let mut easy = generate_easy(&options);

    let _result: Vec<RequestResponse> = listable_check(&mut easy, url.clone(), Some(4), 0, true);

    m1.assert();
    m2.assert();

}

#[test]
fn test_easy_options() {

    // get url of dummy http server
    let url: String = mockito::server_url().clone();

    // create mock server
    let _m1 = mock("HEAD", "/")
        .with_status(201)
        .create();

    let mut options = create_globalopts();

    // modify defaults
    options.http_verb = HttpVerb::Head;
    options.ignore_cert = true;
    options.user_agent = Some(String::from("Mozilla/5.0"));
    options.username = Some(String::from("username"));
    options.password = Some(String::from("password"));
    options.cookies = Some(String::from("cookie"));


    let mut header_list: Vec<String> = Vec::new();
    header_list.push(String::from("User-Agent: Mozilla/5.0"));
    options.headers = Some(header_list);

    let options = Arc::new(options);

    let mut easy = generate_easy(&options);

    let result = make_request(&mut easy, url.clone());

    let mut request = fabricate_request_response(url, false, false);
    request.code = 201;
    request.found_from_listable = false;

    assert_eq!(result, request);

}

#[test]
fn test_proxy_option() {

    // get url of dummy http server
    let url: String = mockito::server_url().clone();

    // create mock server
    let m1 = mock("GET", Matcher::Any)
        .create();

    let mut options = create_globalopts();
    options.proxy_enabled = true;
    options.proxy_address = url.clone();

    let options = Arc::new(options);

    let mut easy = generate_easy(&options);

    let _result = make_request(&mut easy, url);

    m1.assert();

}

#[test]
fn test_post_request() {

    // get url of dummy http server
    let url: String = mockito::server_url().clone();

    // create mock server
    let m1 = mock("POST", Matcher::Any)
        .create();

    let mut options = create_globalopts();
    options.http_verb = HttpVerb::Post;
    let options = Arc::new(options);

    let mut easy = generate_easy(&options);

    let _result = make_request(&mut easy, url.clone());

    m1.assert();

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
