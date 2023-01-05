use actix_web::test;
use serde_json::{json, Value};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::common::encoder::Encoder;
use crate::common::{GetAllDocumentsOptions, Server};

/// This is the basic usage of our API and every other tests uses the content-type application/json
#[actix_rt::test]
async fn add_documents_test_json_content_types() {
    let document = json!([
        {
            "id": 1,
            "content": "Bouvier Bernois",
        }
    ]);

    // this is a what is expected and should work
    let server = Server::new().await;
    let app = server.init_web_app().await;

    // post
    let req = test::TestRequest::post()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "application/json"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 202);
    assert_eq!(response["taskUid"], 0);

    // put
    let req = test::TestRequest::put()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "application/json"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 202);
    assert_eq!(response["taskUid"], 1);
}

/// Here we try to send a single document instead of an array with a single document inside.
#[actix_rt::test]
async fn add_single_document_test_json_content_types() {
    let document = json!({
        "id": 1,
        "content": "Bouvier Bernois",
    });

    // this is a what is expected and should work
    let server = Server::new().await;
    let app = server.init_web_app().await;

    // post
    let req = test::TestRequest::post()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "application/json"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 202);
    assert_eq!(response["taskUid"], 0);

    // put
    let req = test::TestRequest::put()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "application/json"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 202);
    assert_eq!(response["taskUid"], 1);
}

/// Here we try sending encoded (compressed) document request
#[actix_rt::test]
async fn add_single_document_gzip_encoded() {
    let document = json!({
        "id": 1,
        "content": "Bouvier Bernois",
    });

    // this is a what is expected and should work
    let server = Server::new().await;
    let app = server.init_web_app().await;
    // post
    let document = serde_json::to_string(&document).unwrap();
    let encoder = Encoder::Gzip;
    let req = test::TestRequest::post()
        .uri("/indexes/dog/documents")
        .set_payload(encoder.encode(document.clone()))
        .insert_header(("content-type", "application/json"))
        .insert_header(encoder.header().unwrap())
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 202);
    assert_eq!(response["taskUid"], 0);

    // put
    let req = test::TestRequest::put()
        .uri("/indexes/dog/documents")
        .set_payload(encoder.encode(document))
        .insert_header(("content-type", "application/json"))
        .insert_header(encoder.header().unwrap())
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 202);
    assert_eq!(response["taskUid"], 1);
}

/// Here we try document request with every encoding
#[actix_rt::test]
async fn add_single_document_with_every_encoding() {
    let document = json!({
        "id": 1,
        "content": "Bouvier Bernois",
    });

    // this is a what is expected and should work
    let server = Server::new().await;
    let app = server.init_web_app().await;
    // post
    let document = serde_json::to_string(&document).unwrap();

    for (task_uid, encoder) in Encoder::iterator().enumerate() {
        let mut req = test::TestRequest::post()
            .uri("/indexes/dog/documents")
            .set_payload(encoder.encode(document.clone()))
            .insert_header(("content-type", "application/json"));
        req = match encoder.header() {
            Some(header) => req.insert_header(header),
            None => req,
        };
        let req = req.to_request();
        let res = test::call_service(&app, req).await;
        let status_code = res.status();
        let body = test::read_body(res).await;
        let response: Value = serde_json::from_slice(&body).unwrap_or_default();
        assert_eq!(status_code, 202);
        assert_eq!(response["taskUid"], task_uid);
    }
}

/// any other content-type is must be refused
#[actix_rt::test]
async fn error_add_documents_test_bad_content_types() {
    let document = json!([
        {
            "id": 1,
            "content": "Leonberg",
        }
    ]);

    let server = Server::new().await;
    let app = server.init_web_app().await;

    // post
    let req = test::TestRequest::post()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "text/plain"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 415);
    assert_eq!(
        response["message"],
        json!(
            r#"The Content-Type `text/plain` is invalid. Accepted values for the Content-Type header are: `application/json`, `application/x-ndjson`, `text/csv`"#
        )
    );
    assert_eq!(response["code"], "invalid_content_type");
    assert_eq!(response["type"], "invalid_request");
    assert_eq!(response["link"], "https://docs.meilisearch.com/errors#invalid-content-type");

    // put
    let req = test::TestRequest::put()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "text/plain"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 415);
    assert_eq!(
        response["message"],
        json!(
            r#"The Content-Type `text/plain` is invalid. Accepted values for the Content-Type header are: `application/json`, `application/x-ndjson`, `text/csv`"#
        )
    );
    assert_eq!(response["code"], "invalid_content_type");
    assert_eq!(response["type"], "invalid_request");
    assert_eq!(response["link"], "https://docs.meilisearch.com/errors#invalid-content-type");
}

/// missing content-type must be refused
#[actix_rt::test]
async fn error_add_documents_test_no_content_type() {
    let document = json!([
        {
            "id": 1,
            "content": "Leonberg",
        }
    ]);

    let server = Server::new().await;
    let app = server.init_web_app().await;

    // post
    let req = test::TestRequest::post()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 415);
    assert_eq!(
        response["message"],
        json!(
            r#"A Content-Type header is missing. Accepted values for the Content-Type header are: `application/json`, `application/x-ndjson`, `text/csv`"#
        )
    );
    assert_eq!(response["code"], "missing_content_type");
    assert_eq!(response["type"], "invalid_request");
    assert_eq!(response["link"], "https://docs.meilisearch.com/errors#missing-content-type");

    // put
    let req = test::TestRequest::put()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 415);
    assert_eq!(
        response["message"],
        json!(
            r#"A Content-Type header is missing. Accepted values for the Content-Type header are: `application/json`, `application/x-ndjson`, `text/csv`"#
        )
    );
    assert_eq!(response["code"], "missing_content_type");
    assert_eq!(response["type"], "invalid_request");
    assert_eq!(response["link"], "https://docs.meilisearch.com/errors#missing-content-type");
}

#[actix_rt::test]
async fn error_add_malformed_csv_documents() {
    let document = "id, content\n1234, hello, world\n12, hello world";

    let server = Server::new().await;
    let app = server.init_web_app().await;

    // post
    let req = test::TestRequest::post()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "text/csv"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 400);
    assert_eq!(
        response["message"],
        json!(
            r#"The `csv` payload provided is malformed: `CSV error: record 1 (line: 2, byte: 12): found record with 3 fields, but the previous record has 2 fields`."#
        )
    );
    assert_eq!(response["code"], json!("malformed_payload"));
    assert_eq!(response["type"], json!("invalid_request"));
    assert_eq!(response["link"], json!("https://docs.meilisearch.com/errors#malformed-payload"));

    // put
    let req = test::TestRequest::put()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "text/csv"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 400);
    assert_eq!(
        response["message"],
        json!(
            r#"The `csv` payload provided is malformed: `CSV error: record 1 (line: 2, byte: 12): found record with 3 fields, but the previous record has 2 fields`."#
        )
    );
    assert_eq!(response["code"], json!("malformed_payload"));
    assert_eq!(response["type"], json!("invalid_request"));
    assert_eq!(response["link"], json!("https://docs.meilisearch.com/errors#malformed-payload"));
}

#[actix_rt::test]
async fn error_add_malformed_json_documents() {
    let document = r#"[{"id": 1}, {id: 2}]"#;

    let server = Server::new().await;
    let app = server.init_web_app().await;

    // post
    let req = test::TestRequest::post()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "application/json"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 400);
    assert_eq!(
        response["message"],
        json!(
            r#"The `json` payload provided is malformed. `Couldn't serialize document value: key must be a string at line 1 column 14`."#
        )
    );
    assert_eq!(response["code"], json!("malformed_payload"));
    assert_eq!(response["type"], json!("invalid_request"));
    assert_eq!(response["link"], json!("https://docs.meilisearch.com/errors#malformed-payload"));

    // put
    let req = test::TestRequest::put()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "application/json"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 400);
    assert_eq!(
        response["message"],
        json!(
            r#"The `json` payload provided is malformed. `Couldn't serialize document value: key must be a string at line 1 column 14`."#
        )
    );
    assert_eq!(response["code"], json!("malformed_payload"));
    assert_eq!(response["type"], json!("invalid_request"));
    assert_eq!(response["link"], json!("https://docs.meilisearch.com/errors#malformed-payload"));

    // truncate

    // length = 100
    let long = "0123456789".repeat(10);

    let document = format!("\"{}\"", long);
    let req = test::TestRequest::put()
        .uri("/indexes/dog/documents")
        .set_payload(document)
        .insert_header(("content-type", "application/json"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 400);
    assert_eq!(
        response["message"],
        json!(
            r#"The `json` payload provided is malformed. `Couldn't serialize document value: data are neither an object nor a list of objects`."#
        )
    );
    assert_eq!(response["code"], json!("malformed_payload"));
    assert_eq!(response["type"], json!("invalid_request"));
    assert_eq!(response["link"], json!("https://docs.meilisearch.com/errors#malformed-payload"));

    // add one more char to the long string to test if the truncating works.
    let document = format!("\"{}m\"", long);
    let req = test::TestRequest::put()
        .uri("/indexes/dog/documents")
        .set_payload(document)
        .insert_header(("content-type", "application/json"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 400);
    assert_eq!(
        response["message"],
        json!("The `json` payload provided is malformed. `Couldn't serialize document value: data are neither an object nor a list of objects`.")
    );
    assert_eq!(response["code"], json!("malformed_payload"));
    assert_eq!(response["type"], json!("invalid_request"));
    assert_eq!(response["link"], json!("https://docs.meilisearch.com/errors#malformed-payload"));
}

#[actix_rt::test]
async fn error_add_malformed_ndjson_documents() {
    let document = "{\"id\": 1}\n{id: 2}";

    let server = Server::new().await;
    let app = server.init_web_app().await;

    // post
    let req = test::TestRequest::post()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "application/x-ndjson"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 400);
    assert_eq!(
        response["message"],
        json!(
            r#"The `ndjson` payload provided is malformed. `Couldn't serialize document value: key must be a string at line 2 column 2`."#
        )
    );
    assert_eq!(response["code"], json!("malformed_payload"));
    assert_eq!(response["type"], json!("invalid_request"));
    assert_eq!(response["link"], json!("https://docs.meilisearch.com/errors#malformed-payload"));

    // put
    let req = test::TestRequest::put()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "application/x-ndjson"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 400);
    assert_eq!(
        response["message"],
        json!("The `ndjson` payload provided is malformed. `Couldn't serialize document value: key must be a string at line 2 column 2`.")
    );
    assert_eq!(response["code"], json!("malformed_payload"));
    assert_eq!(response["type"], json!("invalid_request"));
    assert_eq!(response["link"], json!("https://docs.meilisearch.com/errors#malformed-payload"));
}

#[actix_rt::test]
async fn error_add_missing_payload_csv_documents() {
    let document = "";

    let server = Server::new().await;
    let app = server.init_web_app().await;

    // post
    let req = test::TestRequest::post()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "text/csv"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 400);
    assert_eq!(response["message"], json!(r#"A csv payload is missing."#));
    assert_eq!(response["code"], json!("missing_payload"));
    assert_eq!(response["type"], json!("invalid_request"));
    assert_eq!(response["link"], json!("https://docs.meilisearch.com/errors#missing-payload"));

    // put
    let req = test::TestRequest::put()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "text/csv"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 400);
    assert_eq!(response["message"], json!(r#"A csv payload is missing."#));
    assert_eq!(response["code"], json!("missing_payload"));
    assert_eq!(response["type"], json!("invalid_request"));
    assert_eq!(response["link"], json!("https://docs.meilisearch.com/errors#missing-payload"));
}

#[actix_rt::test]
async fn error_add_missing_payload_json_documents() {
    let document = "";

    let server = Server::new().await;
    let app = server.init_web_app().await;

    // post
    let req = test::TestRequest::post()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "application/json"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 400);
    assert_eq!(response["message"], json!(r#"A json payload is missing."#));
    assert_eq!(response["code"], json!("missing_payload"));
    assert_eq!(response["type"], json!("invalid_request"));
    assert_eq!(response["link"], json!("https://docs.meilisearch.com/errors#missing-payload"));

    // put
    let req = test::TestRequest::put()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "application/json"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 400);
    assert_eq!(response["message"], json!(r#"A json payload is missing."#));
    assert_eq!(response["code"], json!("missing_payload"));
    assert_eq!(response["type"], json!("invalid_request"));
    assert_eq!(response["link"], json!("https://docs.meilisearch.com/errors#missing-payload"));
}

#[actix_rt::test]
async fn error_add_missing_payload_ndjson_documents() {
    let document = "";

    let server = Server::new().await;
    let app = server.init_web_app().await;

    // post
    let req = test::TestRequest::post()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "application/x-ndjson"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 400);
    assert_eq!(response["message"], json!(r#"A ndjson payload is missing."#));
    assert_eq!(response["code"], json!("missing_payload"));
    assert_eq!(response["type"], json!("invalid_request"));
    assert_eq!(response["link"], json!("https://docs.meilisearch.com/errors#missing-payload"));

    // put
    let req = test::TestRequest::put()
        .uri("/indexes/dog/documents")
        .set_payload(document.to_string())
        .insert_header(("content-type", "application/x-ndjson"))
        .to_request();
    let res = test::call_service(&app, req).await;
    let status_code = res.status();
    let body = test::read_body(res).await;
    let response: Value = serde_json::from_slice(&body).unwrap_or_default();
    assert_eq!(status_code, 400);
    assert_eq!(response["message"], json!(r#"A ndjson payload is missing."#));
    assert_eq!(response["code"], json!("missing_payload"));
    assert_eq!(response["type"], json!("invalid_request"));
    assert_eq!(response["link"], json!("https://docs.meilisearch.com/errors#missing-payload"));
}

#[actix_rt::test]
async fn add_documents_no_index_creation() {
    let server = Server::new().await;
    let index = server.index("test");

    let documents = json!([
        {
            "id": 1,
            "content": "foo",
        }
    ]);

    let (response, code) = index.add_documents(documents, None).await;
    assert_eq!(code, 202);
    assert_eq!(response["taskUid"], 0);
    /*
     * currently we don’t check these field to stay ISO with meilisearch
     * assert_eq!(response["status"], "pending");
     * assert_eq!(response["meta"]["type"], "DocumentsAddition");
     * assert_eq!(response["meta"]["format"], "Json");
     * assert_eq!(response["meta"]["primaryKey"], Value::Null);
     * assert!(response.get("enqueuedAt").is_some());
     */

    index.wait_task(0).await;

    let (response, code) = index.get_task(0).await;
    assert_eq!(code, 200);
    assert_eq!(response["status"], "succeeded");
    assert_eq!(response["uid"], 0);
    assert_eq!(response["type"], "documentAdditionOrUpdate");
    assert_eq!(response["details"]["receivedDocuments"], 1);
    assert_eq!(response["details"]["indexedDocuments"], 1);

    let processed_at =
        OffsetDateTime::parse(response["finishedAt"].as_str().unwrap(), &Rfc3339).unwrap();
    let enqueued_at =
        OffsetDateTime::parse(response["enqueuedAt"].as_str().unwrap(), &Rfc3339).unwrap();
    assert!(processed_at > enqueued_at);

    // index was created, and primary key was inferred.
    let (response, code) = index.get().await;
    assert_eq!(code, 200);
    assert_eq!(response["primaryKey"], "id");
}

#[actix_rt::test]
async fn error_document_add_create_index_bad_uid() {
    let server = Server::new().await;
    let index = server.index("883  fj!");
    let (response, code) = index.add_documents(json!([{"id": 1}]), None).await;

    let expected_response = json!({
        "message": "`883  fj!` is not a valid index uid. Index uid can be an integer or a string containing only alphanumeric characters, hyphens (-) and underscores (_).",
        "code": "invalid_index_uid",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#invalid-index-uid"
    });

    assert_eq!(code, 400);
    assert_eq!(response, expected_response);
}

#[actix_rt::test]
async fn document_addition_with_primary_key() {
    let server = Server::new().await;
    let index = server.index("test");

    let documents = json!([
        {
            "primary": 1,
            "content": "foo",
        }
    ]);
    let (response, code) = index.add_documents(documents, Some("primary")).await;
    assert_eq!(code, 202, "response: {}", response);

    index.wait_task(0).await;

    let (response, code) = index.get_task(0).await;
    assert_eq!(code, 200);
    assert_eq!(response["status"], "succeeded");
    assert_eq!(response["uid"], 0);
    assert_eq!(response["type"], "documentAdditionOrUpdate");
    assert_eq!(response["details"]["receivedDocuments"], 1);
    assert_eq!(response["details"]["indexedDocuments"], 1);

    let (response, code) = index.get().await;
    assert_eq!(code, 200);
    assert_eq!(response["primaryKey"], "primary");
}

#[actix_rt::test]
async fn replace_document() {
    let server = Server::new().await;
    let index = server.index("test");

    let documents = json!([
        {
            "doc_id": 1,
            "content": "foo",
        }
    ]);

    let (response, code) = index.add_documents(documents, None).await;
    assert_eq!(code, 202, "response: {}", response);

    index.wait_task(0).await;

    let documents = json!([
        {
            "doc_id": 1,
            "other": "bar",
        }
    ]);

    let (_response, code) = index.add_documents(documents, None).await;
    assert_eq!(code, 202);

    index.wait_task(1).await;

    let (response, code) = index.get_task(1).await;
    assert_eq!(code, 200);
    assert_eq!(response["status"], "succeeded");

    let (response, code) = index.get_document(1, None).await;
    assert_eq!(code, 200);
    assert_eq!(response.to_string(), r##"{"doc_id":1,"other":"bar"}"##);
}

#[actix_rt::test]
async fn add_no_documents() {
    let server = Server::new().await;
    let index = server.index("test");
    let (_response, code) = index.add_documents(json!([]), None).await;
    assert_eq!(code, 202);
}

#[actix_rt::test]
async fn add_larger_dataset() {
    let server = Server::new().await;
    let index = server.index("test");
    let update_id = index.load_test_set().await;
    let (response, code) = index.get_task(update_id).await;
    assert_eq!(code, 200);
    assert_eq!(response["status"], "succeeded");
    assert_eq!(response["type"], "documentAdditionOrUpdate");
    assert_eq!(response["details"]["indexedDocuments"], 77);
    assert_eq!(response["details"]["receivedDocuments"], 77);
    let (response, code) = index
        .get_all_documents(GetAllDocumentsOptions { limit: Some(1000), ..Default::default() })
        .await;
    assert_eq!(code, 200, "failed with `{}`", response);
    assert_eq!(response["results"].as_array().unwrap().len(), 77);

    // x-ndjson add large test
    let server = Server::new().await;
    let index = server.index("test");
    let update_id = index.load_test_set_ndjson().await;
    let (response, code) = index.get_task(update_id).await;
    assert_eq!(code, 200);
    assert_eq!(response["status"], "succeeded");
    assert_eq!(response["type"], "documentAdditionOrUpdate");
    assert_eq!(response["details"]["indexedDocuments"], 77);
    assert_eq!(response["details"]["receivedDocuments"], 77);
    let (response, code) = index
        .get_all_documents(GetAllDocumentsOptions { limit: Some(1000), ..Default::default() })
        .await;
    assert_eq!(code, 200, "failed with `{}`", response);
    assert_eq!(response["results"].as_array().unwrap().len(), 77);
}

#[actix_rt::test]
async fn error_add_documents_bad_document_id() {
    let server = Server::new().await;
    let index = server.index("test");
    index.create(Some("docid")).await;
    let documents = json!([
        {
            "docid": "foo & bar",
            "content": "foobar"
        }
    ]);
    index.add_documents(documents, None).await;
    index.wait_task(1).await;
    let (response, code) = index.get_task(1).await;
    assert_eq!(code, 200);
    assert_eq!(response["status"], json!("failed"));
    assert_eq!(
        response["error"]["message"],
        json!(
            r#"Document identifier `"foo & bar"` is invalid. A document identifier can be of type integer or string, only composed of alphanumeric characters (a-z A-Z 0-9), hyphens (-) and underscores (_)."#
        )
    );
    assert_eq!(response["error"]["code"], json!("invalid_document_id"));
    assert_eq!(response["error"]["type"], json!("invalid_request"));
    assert_eq!(
        response["error"]["link"],
        json!("https://docs.meilisearch.com/errors#invalid-document-id")
    );
}

#[actix_rt::test]
async fn error_add_documents_missing_document_id() {
    let server = Server::new().await;
    let index = server.index("test");
    index.create(Some("docid")).await;
    let documents = json!([
        {
            "id": "11",
            "content": "foobar"
        }
    ]);
    index.add_documents(documents, None).await;
    index.wait_task(1).await;
    let (response, code) = index.get_task(1).await;
    assert_eq!(code, 200);
    assert_eq!(response["status"], "failed");
    assert_eq!(
        response["error"]["message"],
        json!(r#"Document doesn't have a `docid` attribute: `{"id":"11","content":"foobar"}`."#)
    );
    assert_eq!(response["error"]["code"], json!("missing_document_id"));
    assert_eq!(response["error"]["type"], json!("invalid_request"));
    assert_eq!(
        response["error"]["link"],
        json!("https://docs.meilisearch.com/errors#missing-document-id")
    );
}

#[actix_rt::test]
#[ignore] // // TODO: Fix in an other PR: this does not provoke any error.
async fn error_document_field_limit_reached() {
    let server = Server::new().await;
    let index = server.index("test");

    index.create(Some("id")).await;

    let mut big_object = std::collections::HashMap::new();
    big_object.insert("id".to_owned(), "wow");
    for i in 0..65535 {
        let key = i.to_string();
        big_object.insert(key, "I am a text!");
    }

    let documents = json!([big_object]);

    let (_response, code) = index.update_documents(documents, Some("id")).await;
    assert_eq!(code, 202);

    index.wait_task(0).await;
    let (response, code) = index.get_task(0).await;
    assert_eq!(code, 200);
    // Documents without a primary key are not accepted.
    assert_eq!(response["status"], "failed");

    let expected_error = json!({
        "message": "A document cannot contain more than 65,535 fields.",
        "code": "document_fields_limit_reached",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#document-fields-limit-reached"
    });

    assert_eq!(response["error"], expected_error);
}

#[actix_rt::test]
async fn add_documents_invalid_geo_field() {
    let server = Server::new().await;
    let index = server.index("test");
    index.create(Some("id")).await;
    index.update_settings(json!({"sortableAttributes": ["_geo"]})).await;

    let documents = json!([
        {
            "id": "11",
            "_geo": "foobar"
        }
    ]);

    index.add_documents(documents, None).await;
    index.wait_task(2).await;
    let (response, code) = index.get_task(2).await;
    assert_eq!(code, 200);
    assert_eq!(response["status"], "failed");
}

#[actix_rt::test]
async fn error_add_documents_payload_size() {
    let server = Server::new().await;
    let index = server.index("test");
    index.create(Some("id")).await;
    let document = json!(
        {
            "id": "11",
            "content": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec metus erat, consequat in blandit venenatis, ultricies eu ipsum. Etiam luctus elit et mollis ultrices. Nam turpis risus, dictum non eros in, eleifend feugiat elit. Morbi non dolor pulvinar, sagittis mi sed, ultricies lorem. Nulla ultricies sem metus. Donec at suscipit quam, sed elementum mi. Suspendisse potenti. Fusce pharetra turpis tortor, sed eleifend odio dapibus ut. Nulla facilisi. Suspendisse elementum, dui eget aliquet dignissim, ex tellus aliquam nisl, at eleifend nisl metus tempus diam. Mauris fermentum sollicitudin efficitur. Donec dignissim est vitae elit finibus faucibus"
        }
    );
    let documents: Vec<_> = (0..16000).into_iter().map(|_| document.clone()).collect();
    let documents = json!(documents);
    let (response, code) = index.add_documents(documents, None).await;

    let expected_response = json!({
        "message": "The provided payload reached the size limit.",
        "code": "payload_too_large",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#payload-too-large"
    });

    assert_eq!(response, expected_response);
    assert_eq!(code, 413);
}

#[actix_rt::test]
async fn error_primary_key_inference() {
    let server = Server::new().await;
    let index = server.index("test");

    let documents = json!([
        {
            "title": "11",
            "desc": "foobar"
        }
    ]);

    index.add_documents(documents, None).await;
    index.wait_task(0).await;
    let (response, code) = index.get_task(0).await;
    assert_eq!(code, 200);

    insta::assert_json_snapshot!(response, { ".duration" => "[duration]", ".enqueuedAt" => "[date]", ".startedAt" => "[date]", ".finishedAt" => "[date]" },
    @r###"
    {
      "uid": 0,
      "indexUid": "test",
      "status": "failed",
      "type": "documentAdditionOrUpdate",
      "canceledBy": null,
      "details": {
        "receivedDocuments": 1,
        "indexedDocuments": 1
      },
      "error": {
        "message": "The primary key inference process failed because the engine did not find any field ending with `id` in its name. Please specify the primary key manually using the `primaryKey` query parameter.",
        "code": "index_primary_key_no_candidate_found",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#index-primary-key-no-candidate-found"
      },
      "duration": "[duration]",
      "enqueuedAt": "[date]",
      "startedAt": "[date]",
      "finishedAt": "[date]"
    }
    "###);

    let documents = json!([
        {
            "primary_id": "12",
            "object_id": "42",
            "id": "124",
            "title": "11",
            "desc": "foobar"
        }
    ]);

    index.add_documents(documents, None).await;
    index.wait_task(1).await;
    let (response, code) = index.get_task(1).await;
    assert_eq!(code, 200);

    insta::assert_json_snapshot!(response, { ".duration" => "[duration]", ".enqueuedAt" => "[date]", ".startedAt" => "[date]", ".finishedAt" => "[date]" },
    @r###"
    {
      "uid": 1,
      "indexUid": "test",
      "status": "failed",
      "type": "documentAdditionOrUpdate",
      "canceledBy": null,
      "details": {
        "receivedDocuments": 1,
        "indexedDocuments": 1
      },
      "error": {
        "message": "The primary key inference process failed because the engine found 3 fields ending with `id` in their name, such as 'id' and 'object_id'. Please specify the primary key manually using the `primaryKey` query parameter.",
        "code": "index_primary_key_multiple_candidates_found",
        "type": "invalid_request",
        "link": "https://docs.meilisearch.com/errors#index-primary-key-multiple-candidates-found"
      },
      "duration": "[duration]",
      "enqueuedAt": "[date]",
      "startedAt": "[date]",
      "finishedAt": "[date]"
    }
    "###);

    let documents = json!([
        {
            "primary_id": "12",
            "title": "11",
            "desc": "foobar"
        }
    ]);

    index.add_documents(documents, None).await;
    index.wait_task(2).await;
    let (response, code) = index.get_task(2).await;
    assert_eq!(code, 200);

    insta::assert_json_snapshot!(response, { ".duration" => "[duration]", ".enqueuedAt" => "[date]", ".startedAt" => "[date]", ".finishedAt" => "[date]" },
    @r###"
    {
      "uid": 2,
      "indexUid": "test",
      "status": "succeeded",
      "type": "documentAdditionOrUpdate",
      "canceledBy": null,
      "details": {
        "receivedDocuments": 1,
        "indexedDocuments": 1
      },
      "error": null,
      "duration": "[duration]",
      "enqueuedAt": "[date]",
      "startedAt": "[date]",
      "finishedAt": "[date]"
    }
    "###);
}

#[actix_rt::test]
async fn add_documents_with_primary_key_twice() {
    let server = Server::new().await;
    let index = server.index("test");

    let documents = json!([
        {
            "title": "11",
            "desc": "foobar"
        }
    ]);

    index.add_documents(documents.clone(), Some("title")).await;
    index.wait_task(0).await;
    let (response, _code) = index.get_task(0).await;
    assert_eq!(response["status"], "succeeded");

    index.add_documents(documents, Some("title")).await;
    index.wait_task(1).await;
    let (response, _code) = index.get_task(1).await;
    assert_eq!(response["status"], "succeeded");
}

#[actix_rt::test]
async fn batch_several_documents_addition() {
    let server = Server::new().await;
    let index = server.index("test");

    let mut documents: Vec<_> = (0..150usize)
        .into_iter()
        .map(|id| {
            json!(
                {
                    "id": id,
                    "title": "foo",
                    "desc": "bar"
                }
            )
        })
        .collect();

    documents[100] = json!({"title": "error", "desc": "error"});

    // enqueue batch of documents
    let mut waiter = Vec::new();
    for chunk in documents.chunks(30) {
        waiter.push(index.add_documents(json!(chunk), Some("id")));
    }

    // wait first batch of documents to finish
    futures::future::join_all(waiter).await;
    index.wait_task(4).await;

    // run a second completely failing batch
    documents[40] = json!({"title": "error", "desc": "error"});
    documents[70] = json!({"title": "error", "desc": "error"});
    documents[130] = json!({"title": "error", "desc": "error"});
    let mut waiter = Vec::new();
    for chunk in documents.chunks(30) {
        waiter.push(index.add_documents(json!(chunk), Some("id")));
    }
    // wait second batch of documents to finish
    futures::future::join_all(waiter).await;
    index.wait_task(9).await;

    let (response, _code) = index.filtered_tasks(&[], &["failed"]).await;

    // Check if only the 6th task failed
    println!("{}", &response);
    assert_eq!(response["results"].as_array().unwrap().len(), 5);

    // Check if there are exactly 120 documents (150 - 30) in the index;
    let (response, code) = index
        .get_all_documents(GetAllDocumentsOptions { limit: Some(200), ..Default::default() })
        .await;
    assert_eq!(code, 200, "failed with `{}`", response);
    assert_eq!(response["results"].as_array().unwrap().len(), 120);
}