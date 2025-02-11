use serde::{ Serialize, Deserialize};

#[derive(Debug, Deserialize, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserSchema {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserSchema {
    pub name: Option<String>,
    pub email: Option<String>,
}
