// This test is independent, that will make my life is a lot easy, in other word if I make test dependecy it will make my life so hard.
// it better idea for running test independently. you can also make it run squence and paralel but yeah at least it will make your life easy with no inter dependency each other

use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};

pub mod common;

#[test]
fn test_get_rustaceans() {
    // Setup
    let client = Client::new();
    let rustacean1 = common::create_test_rustacean(&client);
    let rustacean2 = common::create_test_rustacean(&client);

    // Test
    let response = client.get(format!("{}/rustaceans", common::APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));

    // Cleanup
    common::delete_test_rustacen(&client, rustacean1);
    common::delete_test_rustacen(&client, rustacean2);
}

#[test]
fn test_create_rustacean() {
    // Test
    let client = Client::new();
    let response = client.post(format!("{}/rustaceans", common::APP_HOST))
        .json(&json!({
            "name": "Foo Bar",
            "email": "foo@bar.com"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Foo Bar",
        "email": "foo@bar.com",
        "created_at": rustacean["created_at"]
    }));

    // Cleanup
    common::delete_test_rustacen(&client, rustacean)
}

#[test]
fn test_view_rustacean() {
    // Setup
    let client = Client::new();   
    let rustacean = common::create_test_rustacean(&client);

    // Test
    let response = client.get(format!("{}/rustaceans/{}", common::APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Foo Bar",
        "email": "foo@bar.com",
        "created_at": rustacean["created_at"]
    }));

    // Cleanup
    common::delete_test_rustacen(&client, rustacean);
}


#[test]
fn test_update_rustacean() {
    // Setup
    let client = Client::new();   
    let rustacean: Value = common::create_test_rustacean(&client);

    // Test
    let response = client.put(format!("{}/rustaceans/{}", common::APP_HOST, rustacean["id"]))
        .json(&json!({
            "name": "Fooz Bar",
            "email": "fooz@bar.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "Fooz Bar",
        "email": "fooz@bar.com",
        "created_at": rustacean["created_at"]
    }));

    // Cleanup
    common::delete_test_rustacen(&client, rustacean);
}

#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let response = client.delete(format!("{}/rustaceans/{}", common::APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}