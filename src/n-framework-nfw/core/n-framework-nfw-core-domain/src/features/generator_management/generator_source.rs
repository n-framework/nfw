#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorSource {
    pub name: String,
    pub url: String,
    pub enabled: bool,
}

impl GeneratorSource {
    /// Creates a new enabled generator source without validation.
    ///
    /// # Note
    /// This constructor does not validate the input. For validated construction,
    /// use `try_new()` which returns a `Result`.
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            enabled: true,
        }
    }

    /// Creates a new enabled generator source with validation.
    ///
    /// # Errors
    /// Returns an error if:
    /// - name is empty
    /// - url is empty
    pub fn try_new(name: String, url: String) -> Result<Self, String> {
        let name = name.trim();
        let url = url.trim();

        if name.is_empty() {
            return Err("name cannot be empty".to_owned());
        }

        if url.is_empty() {
            return Err("url cannot be empty".to_owned());
        }

        Ok(Self {
            name: name.to_owned(),
            url: url.to_owned(),
            enabled: true,
        })
    }

    /// Creates a new disabled generator source without validation.
    ///
    /// # Note
    /// This constructor does not validate the input. For validated construction,
    /// use `try_new_disabled()` which returns a `Result`.
    pub fn new_disabled(name: String, url: String) -> Self {
        Self {
            name,
            url,
            enabled: false,
        }
    }

    /// Creates a new disabled generator source with validation.
    ///
    /// # Errors
    /// Returns an error if:
    /// - name is empty
    /// - url is empty
    pub fn try_new_disabled(name: String, url: String) -> Result<Self, String> {
        let name = name.trim();
        let url = url.trim();

        if name.is_empty() {
            return Err("name cannot be empty".to_owned());
        }

        if url.is_empty() {
            return Err("url cannot be empty".to_owned());
        }

        Ok(Self {
            name: name.to_owned(),
            url: url.to_owned(),
            enabled: false,
        })
    }
}
