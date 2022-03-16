use axum::Json;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::res::Res;
use noxue_compiler::lang;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CodeInfo {
    lang: String,
    code: String,
    input: Option<String>,
}

pub async fn run_code(body: Json<CodeInfo>) -> Res<Value> {
    let timeout = 2;

    let mut res = Res::default();

    let lang = &body.lang;
    let code = &body.code;
    let input = &body.input.clone().unwrap_or_default();

    let out = if lang == "c" {
        match lang::c::run(&code, &input, timeout) {
            Ok(v) => v,
            Err(e) => {
                log::error!("运行出错:{:#?}", e);
                res.set_code(1);
                res.set_msg(&e);
                return res;
            }
        }
    } else if lang == "cpp" {
        match lang::cpp::run(&code, &input, timeout) {
            Ok(v) => v,
            Err(e) => {
                log::error!("运行出错:{:#?}", e);
                res.set_code(1);
                res.set_msg(&e);
                return res;
            }
        }
    } else if lang == "python2" {
        match lang::python2::run(&code, &input, timeout) {
            Ok(v) => v,
            Err(e) => {
                log::error!("运行出错:{:#?}", e);
                res.set_code(1);
                res.set_msg(&e);
                return res;
            }
        }
    } else if lang == "python3" {
        match lang::python3::run(&code, &input, timeout) {
            Ok(v) => v,
            Err(e) => {
                log::error!("运行出错:{:#?}", e);
                res.set_code(1);
                res.set_msg(&e);
                return res;
            }
        }
    } else if lang == "php5" {
        match lang::php56::run(&code, &input, timeout) {
            Ok(v) => v,
            Err(e) => {
                log::error!("运行出错:{:#?}", e);
                res.set_code(1);
                res.set_msg(&e);
                return res;
            }
        }
    } else if lang == "php7" {
        match lang::php7::run(&code, &input, timeout) {
            Ok(v) => v,
            Err(e) => {
                log::error!("运行出错:{:#?}", e);
                res.set_code(1);
                res.set_msg(&e);
                return res;
            }
        }
    } else if lang == "php8" {
        match lang::php8::run(&code, &input, timeout) {
            Ok(v) => v,
            Err(e) => {
                log::error!("运行出错:{:#?}", e);
                res.set_code(1);
                res.set_msg(&e);
                return res;
            }
        }
    } else if lang == "ruby" {
        match lang::ruby::run(&code, &input, timeout) {
            Ok(v) => v,
            Err(e) => {
                log::error!("运行出错:{:#?}", e);
                res.set_code(1);
                res.set_msg(&e);
                return res;
            }
        }
    } else if lang == "golang" {
        match lang::golang::run(&code, &input, timeout) {
            Ok(v) => v,
            Err(e) => {
                log::error!("运行出错:{:#?}", e);
                res.set_code(1);
                res.set_msg(&e);
                return res;
            }
        }
    } else {
        log::error!("不支持该语言:{}", lang);
        res.set_code(2);
        res.set_msg(&format!("暂时不支持该语言:{}", lang));
        return res;
    };

    debug!("ouput:{:#?}", out);
    res.set_data(json!({
        "stdout":out.stdout,
        "stderr":out.stderr,
    }));

    res
}
