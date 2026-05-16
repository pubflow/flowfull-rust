use std::path::PathBuf;

use reqwest::{Method, header::HeaderMap};
use serde::de::DeserializeOwned;

use crate::{Result, client::FlowfullClient, request::RequestOptions};

#[derive(Debug, Clone)]
pub enum UploadFile {
    Path(PathBuf),
    Bytes {
        bytes: Vec<u8>,
        file_name: String,
        mime: Option<String>,
    },
}

impl UploadFile {
    pub fn path(path: impl Into<PathBuf>) -> Self {
        Self::Path(path.into())
    }

    pub fn bytes(bytes: impl Into<Vec<u8>>, file_name: impl Into<String>) -> Self {
        Self::Bytes {
            bytes: bytes.into(),
            file_name: file_name.into(),
            mime: None,
        }
    }

    pub fn bytes_with_mime(
        bytes: impl Into<Vec<u8>>,
        file_name: impl Into<String>,
        mime: impl Into<String>,
    ) -> Self {
        Self::Bytes {
            bytes: bytes.into(),
            file_name: file_name.into(),
            mime: Some(mime.into()),
        }
    }
}

pub struct UploadBuilder {
    client: FlowfullClient,
    endpoint: String,
    file: UploadFile,
    field: String,
    file_name: Option<String>,
    headers: HeaderMap,
    form_fields: Vec<(String, String)>,
    options: RequestOptions,
}

impl UploadBuilder {
    pub(crate) fn new(client: FlowfullClient, endpoint: String, file: UploadFile) -> Self {
        Self {
            client,
            endpoint,
            file,
            field: "file".to_string(),
            file_name: None,
            headers: HeaderMap::new(),
            form_fields: Vec::new(),
            options: RequestOptions::default(),
        }
    }

    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.field = field.into();
        self
    }

    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    pub fn form_field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.form_fields.push((key.into(), value.into()));
        self
    }

    pub fn header(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self> {
        let key = reqwest::header::HeaderName::from_bytes(key.as_ref().as_bytes())
            .map_err(|err| crate::FlowfullError::Config(format!("invalid header name: {err}")))?;
        let value = reqwest::header::HeaderValue::from_str(value.as_ref()).map_err(|err| {
            crate::FlowfullError::Config(format!("invalid header value for upload option: {err}"))
        })?;
        self.headers.insert(key, value);
        Ok(self)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }

    pub async fn send<T>(self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut form = reqwest::multipart::Form::new();
        for (key, value) in self.form_fields {
            form = form.text(key, value);
        }

        let part = match self.file {
            UploadFile::Path(path) => {
                let bytes = tokio::fs::read(&path).await?;
                let name = self
                    .file_name
                    .or_else(|| {
                        path.file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                    })
                    .unwrap_or_else(|| "file".to_string());
                let mime = mime_guess::from_path(&path)
                    .first_or_octet_stream()
                    .to_string();
                reqwest::multipart::Part::bytes(bytes)
                    .file_name(name)
                    .mime_str(&mime)?
            }
            UploadFile::Bytes {
                bytes,
                file_name,
                mime,
            } => {
                let name = self.file_name.unwrap_or(file_name);
                let part = reqwest::multipart::Part::bytes(bytes).file_name(name);
                if let Some(mime) = mime {
                    part.mime_str(&mime)?
                } else {
                    part
                }
            }
        };

        form = form.part(self.field, part);

        let mut options = self.options;
        for (key, value) in self.headers.iter() {
            options.headers.insert(key.clone(), value.clone());
        }

        let response = self
            .client
            .raw_multipart_request(Method::POST, &self.endpoint, form, options)
            .await?;
        self.client.parse_json_response(response)
    }
}
