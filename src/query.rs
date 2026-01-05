#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Query {
    pairs: Vec<(String, String)>,
}

impl Query {
    pub fn new() -> Self {
        Self { pairs: vec![] }
    }

    pub fn push(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.pairs.push((key.into(), value.into()));
        self
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.pairs.push((key.into(), value.into()));
        self
    }

    pub fn extend<I, K, V>(&mut self, pairs: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.pairs
            .extend(pairs.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }

    pub fn include(mut self, include: impl AsRef<str>) -> Self {
        self.pairs
            .push(("include".to_string(), include.as_ref().to_string()));
        self
    }

    pub fn fields<I, S>(mut self, resource_type: impl AsRef<str>, fields: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let key = format!("fields[{}]", resource_type.as_ref());
        let value = fields
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<_>>()
            .join(",");
        self.pairs.push((key, value));
        self
    }

    pub fn filter(mut self, name: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        let key = format!("filter[{}]", name.as_ref());
        self.pairs.push((key, value.as_ref().to_string()));
        self
    }

    pub fn push_many<I, V>(mut self, key: impl Into<String>, values: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<String>,
    {
        let key = key.into();
        for value in values {
            self.pairs.push((key.clone(), value.into()));
        }
        self
    }

    pub fn push_array<I, V>(mut self, key: impl AsRef<str>, values: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<String>,
    {
        let key = format!("{}[]", key.as_ref());
        for value in values {
            self.pairs.push((key.clone(), value.into()));
        }
        self
    }

    pub fn sort(mut self, sort: impl AsRef<str>) -> Self {
        self.pairs
            .push(("sort".to_string(), sort.as_ref().to_string()));
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.pairs.push(("limit".to_string(), limit.to_string()));
        self
    }

    pub fn cursor(mut self, cursor: impl AsRef<str>) -> Self {
        self.pairs
            .push(("cursor".to_string(), cursor.as_ref().to_string()));
        self
    }

    pub fn page_param_limit(mut self, limit: i64) -> Self {
        self.pairs
            .push(("page[limit]".to_string(), limit.to_string()));
        self
    }

    pub fn page_param_cursor(mut self, cursor: impl AsRef<str>) -> Self {
        self.pairs
            .push(("page[cursor]".to_string(), cursor.as_ref().to_string()));
        self
    }

    pub fn page_limit(self, limit: i64) -> Self {
        self.limit(limit)
    }

    pub fn page_cursor(self, cursor: impl AsRef<str>) -> Self {
        self.cursor(cursor)
    }

    pub fn build(self) -> Vec<(String, String)> {
        self.pairs
    }
}

impl From<Vec<(String, String)>> for Query {
    fn from(value: Vec<(String, String)>) -> Self {
        Self { pairs: value }
    }
}
