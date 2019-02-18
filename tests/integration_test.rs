use code_review_bot;
#[macro_use]
extern crate serde_derive;
use mockito;

use actix_web::{http, test, HttpMessage};
use mockito::mock;

#[derive(Deserialize, Serialize)]
struct FormData {
    text: String,
    token: String,
}

// pub struct PRResult {
//   html_url: String,
//   title: String,
//   body: String,
//   state: PRState,
//   merged: bool,
//   review_comments: u32,
//   additions: u32,
//   deletions: u32,

//   user: User,
// }

fn search_mock() -> mockito::Mock {
    let json = r#"
        {
            "html_url": "https:://github.com/facebook/react/pulls/123",
            "title": "React Pull Request",
            "body": "Merge this, please",
            "state": "open",
            "merged": false,
            "review_comments": 1,
            "additions": 42,
            "deletions": 1,
            "user": {
                "login": "joeyjoejoejr"
            }
        }
    "#;
    mock("GET", "/repos/facebook/react/pulls/123")
        .with_status(200)
        .with_body(json)
        .create()
}

// #[test]
// fn accept_webhook_with_repo_url() {
//     let _m = search_mock();
//     let form_data = FormData {
//         text: "https://github.com/facebook/react/pulls/123".to_string(),
//         token: "test_token".to_string(),
//     };

//     let mut server = test::TestServer::with_factory(|| {
//         code_review_bot::application(mockito::SERVER_URL, "token").unwrap()
//     });
//     let request = server
//         .client(http::Method::POST, "/review")
//         .content_type("application/x-www-form-urlencoded")
//         .form(form_data)
//         .unwrap();
//     let response = server.execute(request.send()).unwrap();

//     assert!(response.status().is_success());
//     let bytes = server.execute(response.body()).unwrap();
//     let body = std::str::from_utf8(&bytes).unwrap();

//     assert!(body.contains("(+42 -1) https:://github.com/facebook/react/pulls/123 by joeyjoejoejr"));
// }
