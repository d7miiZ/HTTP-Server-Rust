use super::function::Function;

pub struct Request {
    function: Function,
    path: String,
    query: Option<String>,
}

