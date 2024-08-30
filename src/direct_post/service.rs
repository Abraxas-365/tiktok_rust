use super::{
    MediaType, PhotoInitRequest, PhotoInitRequestBuilder, PostInfo, PostMode, PostStatusData,
    PostStatusResponse, Source, SourceInfoBuilder, VideoInitRequest, VideoInitRequestBuilder,
    VideoInitResponse, VideoInitResponseData,
};
use crate::error::{ErrorResponse, TikTokApiError};
use reqwest::Client;
use serde_json::json;
use tokio::{fs::File, io::AsyncReadExt};

pub struct Service {
    base_url: String,
}

impl Service {
    /// Creates a new instance of the Service.
    pub fn new() -> Self {
        Self {
            base_url: String::from("https://open.tiktokapis.com"),
        }
    }

    /// Sets a custom base URL for the Service.
    ///
    /// # Arguments
    ///
    /// * `base_url` - A string slice that holds the custom base URL.
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Initializes a video post on TikTok.
    ///
    /// # Arguments
    ///
    /// * `token` - The API token.
    /// * `video_init_request` - The request data for initializing the video post.
    ///
    /// # Returns
    ///
    /// * `Result<VideoInitResponseData, TikTokApiError>` - The response data or an error.
    pub async fn post_video(
        &self,
        token: &str,
        video_init_request: VideoInitRequest,
    ) -> Result<VideoInitResponseData, TikTokApiError> {
        let url = format!("{}/v2/post/publish/video/init/", self.base_url);
        let client = Client::new();

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json; charset=UTF-8")
            .json(&video_init_request)
            .send()
            .await
            .map_err(|e| TikTokApiError::RequestFailed(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| TikTokApiError::ResponseReadFailed(e.to_string()))?;

        let video_init_response: VideoInitResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && video_init_response.error.code == "ok" {
            Ok(video_init_response.data)
        } else {
            Err(TikTokApiError::from(video_init_response.error))
        }
    }

    /// Uploads a video file to the provided upload URL.
    ///
    /// # Arguments
    ///
    /// * `upload_url` - The URL to which the video file should be uploaded.
    /// * `file_path` - The path to the video file on the local filesystem.
    ///
    /// # Returns
    ///
    /// * `Result<(), TikTokApiError>` - An empty result or an error.
    pub async fn upload_video(
        &self,
        upload_url: &str,
        file_path: &str,
    ) -> Result<(), TikTokApiError> {
        let mut file = File::open(file_path)
            .await
            .map_err(|e| TikTokApiError::RequestFailed(e.to_string()))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .await
            .map_err(|e| TikTokApiError::ResponseReadFailed(e.to_string()))?;

        let client = Client::new();

        let response = client
            .put(upload_url)
            .header(
                "Content-Range",
                format!("bytes 0-{}/{}", buffer.len() - 1, buffer.len()),
            )
            .header("Content-Type", "video/mp4")
            .body(buffer)
            .send()
            .await
            .map_err(|e| TikTokApiError::RequestFailed(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let body = response
                .text()
                .await
                .map_err(|e| TikTokApiError::ResponseReadFailed(e.to_string()))?;
            let error_response: ErrorResponse = serde_json::from_str(&body)
                .map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;
            Err(TikTokApiError::from(error_response))
        }
    }

    /// Retrieves the status of a post using the publish ID.
    ///
    /// # Arguments
    ///
    /// * `token` - The API token.
    /// * `publish_id` - The ID of the post whose status is to be retrieved.
    ///
    /// # Returns
    ///
    /// * `Result<PostStatusData, TikTokApiError>` - The status data or an error.
    pub async fn get_post_status(
        &self,
        token: &str,
        publish_id: &str,
    ) -> Result<PostStatusData, TikTokApiError> {
        let url = format!("{}/v2/post/publish/status/fetch/", self.base_url);
        let client = Client::new();

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json; charset=UTF-8")
            .json(&json!({ "publish_id": publish_id }))
            .send()
            .await
            .map_err(|e| TikTokApiError::RequestFailed(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| TikTokApiError::ResponseReadFailed(e.to_string()))?;

        let post_status_response: PostStatusResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && post_status_response.error.code == "ok" {
            Ok(post_status_response.data)
        } else {
            Err(TikTokApiError::from(post_status_response.error))
        }
    }

    /// Simplified function to upload a video from a file.
    ///
    /// This function combines the steps of initializing a video post, uploading the video file,
    /// and checking the post status into a single function call.
    ///
    /// # Arguments
    ///
    /// * `token` - The API token.
    /// * `post_info` - The post information.
    /// * `file_path` - The path to the video file on the local filesystem.
    /// * `video_size` - The size of the video file in bytes.
    /// * `chunk_size` - The size of each chunk to be uploaded in bytes.
    /// * `total_chunk_count` - The total number of chunks to be uploaded.
    ///
    /// # Returns
    ///
    /// * `Result<PostStatusData, TikTokApiError>` - The status data or an error.
    pub async fn upload_video_from_file(
        &self,
        token: &str,
        post_info: PostInfo,
        file_path: &str,
        video_size: u64,
        chunk_size: u64,
        total_chunk_count: u32,
    ) -> Result<PostStatusData, TikTokApiError> {
        let source_info = SourceInfoBuilder::default()
            .source(Source::FileUpload)
            .video_size(Some(video_size))
            .chunk_size(Some(chunk_size))
            .total_chunk_count(Some(total_chunk_count))
            .build()
            .unwrap();

        let video_init_request = VideoInitRequestBuilder::default()
            .post_info(post_info)
            .source_info(source_info)
            .build()
            .unwrap();

        // Call the post_video function
        let response_data = self.post_video(token, video_init_request).await?;

        self.upload_video(&response_data.upload_url, file_path)
            .await?;

        // Check the post status
        self.get_post_status(token, &response_data.publish_id).await
    }

    /// Simplified function to upload a video from a URL.
    ///
    /// This function combines the steps of initializing a video post and checking the post status
    /// into a single function call.
    ///
    /// # Arguments
    ///
    /// * `token` - The API token.
    /// * `post_info` - The post information.
    /// * `video_url` - The URL of the video to be uploaded.
    ///
    /// # Returns
    ///
    /// * `Result<PostStatusData, TikTokApiError>` - The status data or an error.
    pub async fn upload_video_from_url(
        &self,
        token: &str,
        post_info: PostInfo,
        video_url: &str,
    ) -> Result<PostStatusData, TikTokApiError> {
        // Create SourceInfo for PULL_FROM_URL
        let source_info = SourceInfoBuilder::default()
            .source(Source::PullFromUrl)
            .video_url(Some(video_url.to_string()))
            .build()
            .unwrap();

        // Create VideoInitRequest using the generated builder
        let video_init_request = VideoInitRequestBuilder::default()
            .post_info(post_info)
            .source_info(source_info)
            .build()
            .unwrap();

        // Call the post_video function
        let response_data = self.post_video(token, video_init_request).await?;

        // Check the post status
        self.get_post_status(token, &response_data.publish_id).await
    }

    /// Initializes a photo post on TikTok.
    ///
    /// # Arguments
    ///
    /// * `token` - The API token.
    /// * `photo_init_request` - The request data for initializing the photo post.
    ///
    /// # Returns
    ///
    /// * `Result<VideoInitResponseData, TikTokApiError>` - The response data or an error.
    pub async fn post_photo(
        &self,
        token: &str,
        photo_init_request: PhotoInitRequest,
    ) -> Result<VideoInitResponseData, TikTokApiError> {
        let url = format!("{}/v2/post/publish/content/init/", self.base_url);
        let client = Client::new();

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json; charset=UTF-8")
            .json(&photo_init_request)
            .send()
            .await
            .map_err(|e| TikTokApiError::RequestFailed(e.to_string()))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| TikTokApiError::ResponseReadFailed(e.to_string()))?;

        let photo_init_response: VideoInitResponse =
            serde_json::from_str(&body).map_err(|e| TikTokApiError::ParseFailed(e.to_string()))?;

        if status.is_success() && photo_init_response.error.code == "ok" {
            Ok(photo_init_response.data)
        } else {
            Err(TikTokApiError::from(photo_init_response.error))
        }
    }

    /// Simplified function to upload a photo from URLs.
    ///
    /// This function combines the steps of initializing a photo post and checking the post status
    /// into a single function call.
    ///
    /// The first photo will be the cover
    ///
    /// # Arguments
    ///
    /// * `token` - The API token.
    /// * `post_info` - The post information.
    /// * `photo_urls` - The URLs of the photos to be uploaded.
    ///
    /// # Returns
    ///
    /// * `Result<PostStatusData, TikTokApiError>` - The status data or an error.
    pub async fn upload_photo_from_urls(
        &self,
        token: &str,
        post_info: PostInfo,
        photo_urls: Vec<String>,
    ) -> Result<PostStatusData, TikTokApiError> {
        // Create SourceInfo for PULL_FROM_URL
        let source_info = SourceInfoBuilder::default()
            .source(Source::PullFromUrl)
            .photo_images(Some(photo_urls))
            .photo_cover_index(Some(1)) // Assuming the first photo is the cover
            .build()
            .unwrap();

        // Create PhotoInitRequest using the generated builder
        let photo_init_request = PhotoInitRequestBuilder::default()
            .post_info(post_info)
            .source_info(source_info)
            .post_mode(PostMode::DirectPost)
            .media_type(MediaType::Photo)
            .build()
            .unwrap();

        // Call the post_photo function
        let response_data = self.post_photo(token, photo_init_request).await?;

        // Check the post status
        self.get_post_status(token, &response_data.publish_id).await
    }
}
