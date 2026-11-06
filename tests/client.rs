use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use data_for_seo::{DataForSeoClient, SANDBOX_BASE_URL};

#[test]
fn debug_redacts_credentials() {
    let login = "debug-login@example.com";
    let password = "debug-secret-password";
    let token = BASE64_STANDARD.encode(format!("{login}:{password}"));
    let client = DataForSeoClient::new(login, password).sandbox();

    let output = format!("{client:?}");

    assert!(!output.contains(&token), "token leaked: {output}");
    assert!(!output.contains(login), "login leaked: {output}");
    assert!(!output.contains(password), "password leaked: {output}");
    assert!(output.contains(r#"token: "<redacted>""#), "{output}");
    assert!(output.contains(SANDBOX_BASE_URL), "{output}");

    let pretty = format!("{client:#?}");
    assert!(!pretty.contains(&token), "token leaked: {pretty}");
}
