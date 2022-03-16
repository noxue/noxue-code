
use axum::{
    body,
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Res<T = String>
where
    T: Serialize + Default,
{
    code: i32,
    msg: String,
    data: T,
}

impl<T> Default for Res<T>
where
    T: Serialize + Default,
{
    fn default() -> Self {
        Self {
            code: 0,
            msg: "success".to_string(),
            data: Default::default(),
        }
    }
}

impl<T> IntoResponse for Res<T>
where
    T: Serialize + Default,
{
    fn into_response(self) -> Response {
        let mut res = Response::default();
        *res.status_mut() = StatusCode::OK;
        let data = match serde_json::to_string(&self) {
            Ok(v) => v,
            Err(_) => {
                *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                "".to_owned()
            }
        };
        *res.body_mut() = body::boxed(body::Full::from(data));
        res
    }
}

impl<T> Res<T>
where
    T: Serialize + Default,
{
    pub fn set_data(&mut self, data: T) -> &mut Self {
        self.data = data;
        self
    }

    pub fn set_code(&mut self, code: i32) -> &mut Self {
        self.code = code;
        self
    }

    pub fn set_msg(&mut self, msg: &str) -> &mut Self {
        self.msg = msg.to_string();
        self
    }
}
