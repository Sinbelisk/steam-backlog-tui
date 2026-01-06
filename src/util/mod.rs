use std::fmt::format;


pub struct UrlBuilder{
    base_url: String,
    path: String
}

impl UrlBuilder {

    pub fn new(url: impl Into<String>) -> Self {
        Self {
            base_url: url.into(),
            path: String::new()
        }
    }

    pub fn endpoint(mut self, endpoint: impl AsRef<str>) -> Self {
        // Allows both &str and String
        let new_path = endpoint.as_ref().to_owned();

        self.path = new_path;
        self
    }

    pub fn parameter(mut self, param: impl AsRef<str>) -> Self {
        let path_end = self.path.rsplitn(2, "/")
            .next()
            .unwrap_or("");

        // Checks for a first '?' parameter separator
        let separator = if path_end.contains("?") {"&"}
        else {"?"};

        let parsed_param = format!("{}{}", separator, param.as_ref().to_owned());

        self.path.push_str(&parsed_param);
        self
    }

    pub fn parameters(mut self, params: Vec<impl AsRef<str>>) -> Self {
        for param in &params {
            self = self.parameter(param);
        }

        self
    }

    pub fn build(self) -> String {
        format!("{}/{}", self.base_url, self.path)
    }
}
