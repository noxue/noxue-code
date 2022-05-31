use std::time::Duration;

use axum::Json;
use log::debug;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::res::Res;
use noxue_compiler::lang;

#[derive(Serialize, Deserialize)]
struct RunTpl {
    image: String,            // docker iamge 名字
    file: String,             // 代码要保存的文件路径
    prev_cmd: Option<String>, // 写入之前执行的命令，主要用于设置一些变量，给cmd中的命令使用
    cmd: String,              // 保存代码之后要执行的命令
    timeout: i32,             // 容器执行超时时间
    memory: String,           // 允许容器使用的内存,例如:20MB
    cpuset: String,           // 使用的cpu核心
}

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

    let mut tpl = std::fs::read_to_string(file).unwrap();

    // 根据模板自定义文件名，模板中文件名为 regex:正则表达式:后缀 这样的格式就做处理
    let mut run_tpl: RunTpl = serde_json::from_str(&tpl).unwrap();
    if run_tpl.file.starts_with("regex::") {
        // 分割规则字符串
        let mut tpls = run_tpl.file.split("::");
        tpls.next();
        let rule = tpls.next().unwrap();
        let ext = tpls.next().unwrap();

        log::debug!("rule:{}, ext:{}, code:\n{}\n", rule, ext, code);
        // 从提交的代码中去根据正则匹配类名
        let re = Regex::new(rule).unwrap();
        let caps = match re.captures(code) {
            Some(v) => v,
            None => {
                log::error!("用户选择语言出错，或者代码出错");
                res.set_code(5);
                res.set_msg("确认选择的语言是否正确，或代码出错");
                return res;
            }
        };

        // 把匹配到的变量值，设置成系统变量，添加到 tpl.cmd 前面
        // 以 cap1 cap2 cap3 的方式传入脚本
        let mut pre_cmd = String::new(); // 保存设置变量的命令，用于追加到执行 tpl.cmd 中的命令之前
        let mut index = 0;
        for cap in caps.iter() {
            if index == 0 || cap.is_none() {
                index += 1;
                continue;
            }
            log::debug!("{:?}", cap.unwrap());
            let v = cap.unwrap().as_str();
            pre_cmd += &format!("cap{}={}\n", index, v);
            index += 1;
        }
        // 后缀变量
        pre_cmd += &format!("ext={}\n", ext);

        log::debug!("pre_cmd:{}", pre_cmd);

        run_tpl.file = format!("$cap1.$ext");

        run_tpl.prev_cmd = Some(pre_cmd);

        tpl = serde_json::to_string(&run_tpl).unwrap();
    }
    log::debug!("tpl:{:?}", tpl);
    // 调用执行函数获取执行结果
    let out = match lang::run(&tpl, &code, &input) {
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
