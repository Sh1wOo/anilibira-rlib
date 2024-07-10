#[derive(Debug, Clone)]
pub struct Title {
    pub id: i32,
    pub code: String,
    pub names: Names,
    pub description: Option<String>,
    pub blocked: Option<Blocked>,
}

#[derive(Debug, Clone)]
pub struct Names {
    pub ru: String,
    pub en: String,
}

#[derive(Debug, Clone)]
pub struct Blocked {
    pub blocked: bool,
    pub bakanim: bool,
}