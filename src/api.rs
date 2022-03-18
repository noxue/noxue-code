use std::time::Duration;

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
    let mut res = Res::default();

    let lang = &body.lang;
    let code = &body.code;
    let input = &body.input.clone().unwrap_or_default();

    if code.len() > 1024 * 400 {
        res.set_code(1);
        res.set_msg("提交的代码太长，最多允许400KB");
        return res;
    }

    if input.len() > 1024 * 20 {
        res.set_code(1);
        res.set_msg("输入的太长，最多允许20KB");
        return res;
    }

    log::debug!("lang:{}", lang);

    let file = format!("./lang/{}.json", lang);
    // 这里检查一下运行的语言模板文件是否存在，方便返回更加友好的提示信息
    if !std::path::Path::new(&file).exists() {
        res.set_code(2);
        res.set_msg(&format!("暂时不支持该语言:{}", lang));
        return res;
    }

    // 调用执行函数获取执行结果
    let out = match lang::run(&std::fs::read_to_string(file).unwrap(), &code, &input) {
        Ok(v) => v,
        Err(e) => {
            log::error!("运行出错:{:#?}", e);
            res.set_code(1);
            res.set_msg(&e);
            return res;
        }
    };

    debug!("ouput:{:#?}", out);
    res.set_data(json!({
        "stdout":out.stdout,
        "stderr":out.stderr,
    }));

    res
}

// #[test]
// fn test() {
//     let code = r#"
// #include <stdio.h>

// int main(){
//     printf("hello");
//     for(;;);
//     return 0;
// }"#;
//     let tpl = r#"
// {
// "image": "gcc",
// "file": "test.c",
// "cmd": "gcc test.c -o test\nif test -f \"./test\"; then\n./test\nfi",
// "timeout": 50,
// "memory":"20MB"
// }
// "#;

//     for i in 0..100 {
//         std::thread::spawn(move || {
//             let out = lang::run(tpl, code, "");
//             println!("{:?}", out);
//         });
//     }
//     loop {
//         std::thread::sleep(Duration::from_secs(1));
//     }
//     // assert_eq!(out.unwrap().stdout, "hello");
// }
