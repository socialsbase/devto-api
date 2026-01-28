//! Tests using httpmock for mocking API responses.

use devto_api::Client;
use httpmock::prelude::*;
use std::num::NonZeroU32;

#[tokio::test]
async fn test_get_articles() {
    // Start a mock server
    let server = MockServer::start();

    // Create a mock for the articles endpoint
    let articles_mock = server.mock(|when, then| {
        when.method(GET)
            .path("/api/articles")
            .query_param("per_page", "10");
        then.status(200)
            .header("content-type", "application/json")
            .body(
                r#"[
                    {
                        "type_of": "article",
                        "id": 123,
                        "title": "Test Article",
                        "description": "A test article",
                        "readable_publish_date": "Jan 1",
                        "slug": "test-article-123",
                        "path": "/testuser/test-article-123",
                        "url": "https://dev.to/testuser/test-article-123",
                        "comments_count": 5,
                        "public_reactions_count": 10,
                        "collection_id": null,
                        "published_timestamp": "2024-01-01T00:00:00Z",
                        "positive_reactions_count": 10,
                        "cover_image": null,
                        "social_image": "https://dev.to/social.png",
                        "canonical_url": "https://dev.to/testuser/test-article-123",
                        "created_at": "2024-01-01T00:00:00Z",
                        "edited_at": null,
                        "crossposted_at": null,
                        "published_at": "2024-01-01T00:00:00Z",
                        "last_comment_at": "2024-01-01T12:00:00Z",
                        "reading_time_minutes": 3,
                        "tag_list": ["rust", "programming"],
                        "tags": "rust, programming",
                        "user": {
                            "name": "Test User",
                            "username": "testuser",
                            "twitter_username": null,
                            "github_username": "testuser",
                            "user_id": 1,
                            "website_url": null,
                            "profile_image": "https://dev.to/avatar.png",
                            "profile_image_90": "https://dev.to/avatar90.png"
                        }
                    }
                ]"#,
            );
    });

    // Create a client pointing to the mock server
    let client = Client::new(&server.base_url());

    // Make the request
    let per_page = NonZeroU32::new(10);
    let articles = client
        .get_articles(
            None,     // collection_id
            None,     // page
            per_page, // per_page
            None,     // state
            None,     // tag
            None,     // tags
            None,     // tags_exclude
            None,     // top
            None,     // username
        )
        .await
        .unwrap();

    // Verify the mock was called
    articles_mock.assert();

    // Verify the response
    let articles = articles.into_inner();
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].id, 123);
    assert_eq!(articles[0].title, "Test Article");
}

#[tokio::test]
async fn test_get_article_by_id() {
    let server = MockServer::start();

    let article_mock = server.mock(|when, then| {
        when.method(GET).path("/api/articles/456");
        then.status(200)
            .header("content-type", "application/json")
            .body(
                r#"{
                    "type_of": "article",
                    "id": 456,
                    "title": "Detailed Article",
                    "description": "An article with full details",
                    "readable_publish_date": "Feb 15",
                    "slug": "detailed-article-456",
                    "path": "/author/detailed-article-456",
                    "url": "https://dev.to/author/detailed-article-456",
                    "comments_count": 20,
                    "public_reactions_count": 100,
                    "collection_id": null,
                    "published_timestamp": "2024-02-15T10:30:00Z",
                    "positive_reactions_count": 95,
                    "cover_image": "https://dev.to/cover.png",
                    "social_image": "https://dev.to/social.png",
                    "canonical_url": "https://dev.to/author/detailed-article-456",
                    "created_at": "2024-02-14T08:00:00Z",
                    "edited_at": "2024-02-15T09:00:00Z",
                    "crossposted_at": null,
                    "published_at": "2024-02-15T10:30:00Z",
                    "last_comment_at": "2024-02-16T14:00:00Z",
                    "reading_time_minutes": 8,
                    "tag_list": ["webdev", "tutorial"],
                    "tags": "webdev, tutorial",
                    "user": {
                        "name": "Article Author",
                        "username": "author",
                        "twitter_username": "author_twitter",
                        "github_username": "author_github",
                        "user_id": 42,
                        "website_url": "https://author.com",
                        "profile_image": "https://dev.to/author.png",
                        "profile_image_90": "https://dev.to/author90.png"
                    }
                }"#,
            );
    });

    let client = Client::new(&server.base_url());
    let article = client.get_article_by_id(456).await.unwrap();

    article_mock.assert();

    let article = article.into_inner();
    assert_eq!(article.id, 456);
    assert_eq!(article.title, "Detailed Article");
    assert_eq!(article.reading_time_minutes, 8);
}

#[tokio::test]
async fn test_api_error_handling() {
    let server = MockServer::start();

    let error_mock = server.mock(|when, then| {
        when.method(GET).path("/api/articles/999999");
        then.status(404)
            .header("content-type", "application/json")
            .body(r#"{"error": "not found", "status": 404}"#);
    });

    let client = Client::new(&server.base_url());
    let result = client.get_article_by_id(999999).await;

    error_mock.assert();

    assert!(result.is_err());
}
