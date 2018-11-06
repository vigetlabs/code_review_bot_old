extern crate actix_web;
extern crate code_review_bot;
#[macro_use]
extern crate serde_derive;
extern crate mockito;

use actix_web::{http, test, HttpMessage};
use mockito::mock;

#[derive(Deserialize, Serialize)]
struct FormData {
    text: String,
    token: String,
}

fn search_mock() -> mockito::Mock {
    let json = r#"
        {
            "items": [
                {
                    "name": "React",
                    "html_url": "http://github.com/react/react",
                    "description": "I am awesome",
                    "owner": {
                        "login": "react-dude",
                        "html_url": "http://github.com/react-dude"
                    }
                }
            ]
        }
    "#;
    mock("GET", "/search/repositories?q=react&per_page=10")
        .with_status(200)
        .with_body(json)
        .create()
}

#[test]
fn accept_webhook_with_repo_url() {
    let _m = search_mock();
    let form_data = FormData {
        text: "react".to_string(),
        token: "test_token".to_string(),
    };

    let mut server = test::TestServer::with_factory(|| {
        code_review_bot::application(mockito::SERVER_URL.to_string())
    });
    let request = server
        .client(http::Method::POST, "/code_review_bot")
        .content_type("application/x-www-form-urlencoded")
        .form(form_data)
        .unwrap();
    let response = server.execute(request.send()).unwrap();

    assert!(response.status().is_success());
    let bytes = server.execute(response.body()).unwrap();
    let body = std::str::from_utf8(&bytes).unwrap();

    assert!(body.contains("React"));
    assert!(body.contains("I am awesome"));
    assert!(body.contains("react-dude"));
}
