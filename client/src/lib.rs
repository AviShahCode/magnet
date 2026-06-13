use pyo3::prelude::*;

#[pymodule]
mod client {
    use pyo3::prelude::*;
    use reqwest::blocking::Client;
    use serde::Serialize;

    #[pymodule_export]
    use super::admin;
    #[pymodule_export]
    use super::drive;

    // TODO: repetition
    #[derive(Serialize)]
    struct LoginRequest {
        username: String,
        password: String,
    }
    #[derive(Serialize)]
    struct SignupRequest {
        code: String,
        username: String,
        password: String,
    }

    #[pyclass]
    pub struct Connector {
        client: Client,
        #[pyo3(get)]
        base_url: String,
    }

    #[pymethods]
    impl Connector {
        #[new]
        pub fn new(base_url: String) -> PyResult<Connector> {
            let client = Client::builder()
                .cookie_store(true)
                .build()
                .expect("error building client");
            Ok(Connector { client, base_url })
        }

        pub fn ping(&self) -> PyResult<u16> {
            let url = format!("{}/ping", self.base_url);
            Ok(self
                .client
                .get(url)
                .send()
                .expect("error while ping")
                .status()
                .as_u16())
        }

        fn login(&self, username: String, password: String) -> PyResult<u16> {
            let payload = LoginRequest { username, password };
            let url = format!("{}/login", self.base_url);
            let res = self
                .client
                .post(&url)
                .json(&payload)
                .send()
                .expect("error while login");
            Ok(res.status().as_u16())
        }

        fn logout(&self) -> PyResult<u16> {
            let url = format!("{}/logout", self.base_url);
            let res = self.client.get(&url).send().expect("error while logout");
            Ok(res.status().as_u16())
        }

        fn signup(&self, code: String, username: String, password: String) -> PyResult<u16> {
            let payload = SignupRequest {
                code,
                username,
                password,
            };
            let url = format!("{}/signup", self.base_url);
            let res = self
                .client
                .post(&url)
                .json(&payload)
                .send()
                .expect("error while signup");
            Ok(res.status().as_u16())
        }

        #[getter]
        fn drive(&self) -> drive::DriveConnector {
            drive::DriveConnector {
                client: self.client.clone(),
                base_url: format!("{}/drive", self.base_url),
            }
        }

        #[getter]
        fn admin(&self) -> admin::AdminConnector {
            admin::AdminConnector {
                client: self.client.clone(),
                base_url: format!("{}/admin", self.base_url),
            }
        }
    }
}

#[pymodule]
mod drive {
    use base64::prelude::*;
    use pyo3::prelude::*;
    use reqwest::blocking::Client;
    use serde::{Deserialize, Serialize};

    #[pyclass(eq, eq_int)]
    #[derive(Deserialize, PartialEq, Clone, Debug)]
    #[serde(rename_all = "lowercase")]
    pub enum ItemType {
        Folder,
        File,
    }

    #[pyclass]
    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct FolderItem {
        #[pyo3(get)]
        id: String,
        #[pyo3(get)]
        name: String,
        #[pyo3(get)]
        item_type: ItemType,
    }

    #[pyclass]
    pub enum GetResponse {
        File { name: String, content: Vec<u8> },
        Folder { items: Vec<FolderItem> },
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "lowercase")]
    enum GetResponseDto {
        File { name: String, content: String },
        Folder { items: Vec<FolderItem> },
    }

    #[pymethods]
    impl FolderItem {
        pub fn __repr__(&self) -> PyResult<String> {
            Ok(format!(
                "{} {}: {}",
                if self.item_type == ItemType::File {
                    "📄"
                } else {
                    "📁"
                },
                self.name,
                self.id
            ))
        }
    }

    #[pyclass]
    #[derive(Serialize, Debug, Clone)]
    pub struct UploadItem {
        name: String,
        content: Option<Vec<u8>>,
    }

    #[pymethods]
    impl UploadItem {
        #[new]
        pub fn new(name: String, content: Option<Vec<u8>>) -> PyResult<UploadItem> {
            Ok(Self { name, content })
        }
    }

    #[derive(Serialize)]
    struct UploadItemDto {
        name: String,
        content: Option<String>, // Stores the base64 string
    }

    #[pyclass]
    pub struct DriveConnector {
        pub(crate) client: Client,
        #[pyo3(get)]
        pub(crate) base_url: String,
    }

    #[pymethods]
    impl DriveConnector {
        fn get(&self, path: Option<String>) -> PyResult<(u16, GetResponse)> {
            let url = match path {
                Some(p) => format!("{}/{}", self.base_url, p),
                None => self.base_url.clone(),
            };

            let res = self.client.get(&url).send().expect("error while get");
            let status = res.status().as_u16();

            // TODO: convert all panics to py errors
            if !res.status().is_success() {
                return Ok((status, GetResponse::Folder { items: Vec::new() }));
            }

            let dto: GetResponseDto = res.json::<GetResponseDto>().expect("error while get");

            let files = match dto {
                GetResponseDto::File { name, content } => {
                    let decoded_bytes = BASE64_STANDARD
                        .decode(content)
                        .expect("Failed to decode base64 content");

                    GetResponse::File {
                        name,
                        content: decoded_bytes,
                    }
                }
                GetResponseDto::Folder { items } => GetResponse::Folder { items },
            };

            Ok((status, files))
        }

        fn upload(&self, path: Option<String>, item: UploadItem) -> PyResult<(u16, String)> {
            let url = match path {
                Some(p) => format!("{}/{}", self.base_url, p),
                None => self.base_url.clone(),
            };

            let dto = UploadItemDto {
                name: item.name,
                content: item.content.map(|bytes| BASE64_STANDARD.encode(&bytes)),
            };

            let res = self
                .client
                .post(&url)
                .json(&dto)
                .send()
                .expect("error while upload");

            Ok((res.status().as_u16(), res.text().unwrap()))
        }

        fn delete(&self, path: String) -> PyResult<u16> {
            let url = format!("{}/{}", self.base_url, path);
            let res = self.client.delete(&url).send().expect("error while delete");
            Ok(res.status().as_u16())
        }
    }
}

#[pymodule]
mod admin {
    use pyo3::{PyResult, pyclass, pymethods};
    use reqwest::blocking::Client;

    #[pyclass]
    pub struct AdminConnector {
        pub(crate) client: Client,
        #[pyo3(get)]
        pub(crate) base_url: String,
    }

    #[pymethods]
    impl AdminConnector {
        pub fn ping(&self) -> PyResult<u16> {
            let url = format!("{}/ping", self.base_url);
            Ok(self
                .client
                .get(url)
                .send()
                .expect("error while ping")
                .status()
                .as_u16())
        }

        pub fn signup_code(&self) -> PyResult<(u16, String)> {
            let url = format!("{}/signup_code", self.base_url);
            let resp = self.client.get(url).send().expect("error while signup code");
            Ok((resp.status().as_u16(), resp.text().unwrap_or_default()))
        }
    }
}
