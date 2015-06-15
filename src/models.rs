#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub author: Option<String>,
    pub title: Option<String>,
    pub link: Option<String>,
    pub description: Option<String>,
    pub guid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feed {
    pub id: i32,
    pub title: String,
    pub url: String,
}
