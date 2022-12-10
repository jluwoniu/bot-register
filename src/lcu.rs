use std::collections::HashMap;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use log::{debug, error, info};
use reqwest::blocking::Client;
use serde_json::Value;

struct Config {
    port: String,
    token: String,
    name: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let output = Command::new("cmd")
            .creation_flags(0x08000000)
            .args(["/C", "wmic process where caption='LeagueClientUx.exe' get commandline"])
            .output();
        let output = match output {
            Ok(v) => v,
            Err(_) => return Err("获取参数失败"),
        };

        let args = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if args.is_empty() {
            return Err("请先启动游戏!");
        }
        if !args.contains("--remoting-auth-token=") {
            return Err("请以管理员身份运行!");
        }
        let v: Vec<&str> = args.split("--remoting-auth-token=").collect();
        let v: Vec<&str> = v[1].split('"').collect();
        let token = v[0];
        let v: Vec<&str> = args.split("--app-port=").collect();
        let v: Vec<&str> = v[1].split('"').collect();
        let port = v[0];
        println!("token: {}",token);
        println!("port: {}", port);
        return Ok(Config { port: port.to_string(), token: token.to_string(), name: String::from("riot") });
    }
}

pub struct Lcu {
    http2_client: Client,
    http1_client: Client,
    config: Config,
}

impl Lcu {
    pub fn new() -> Result<Lcu,&'static str> {
        let config = Config::new();
        return match config {
            Ok(config) => {
                Ok(Lcu{
                    http2_client: reqwest::blocking::Client::builder().
                        danger_accept_invalid_certs(true).
                        danger_accept_invalid_hostnames(true).
                        http2_prior_knowledge().
                        build().unwrap(),
                    http1_client: reqwest::blocking::Client::builder().
                        danger_accept_invalid_certs(true).
                        danger_accept_invalid_hostnames(true).
                        build().unwrap(),
                    config,
                })
            },
            Err(e) => {Err(e)}
        }
    }


    pub fn get_current_summoner(&self) -> (i64, String) {
        let response = self.http2_client.get(format!("https://127.0.0.1:{}/lol-summoner/v1/current-summoner", self.config.port))
            .basic_auth(&self.config.name, Some(&self.config.token))
            .send()
            .expect("获取召唤师信息失败")
            .json::<HashMap<String, Value>>()
            .expect("解析召唤师信息失败");
        //let body = response.text().expect("");
        let id = response.get("summonerId").expect("获取ID失败").as_i64().expect("转换失败");
        let name = response.get("displayName").expect("获取Name失败").to_string();
        return (id, name);
    }

    pub fn get_machine_id(&self) -> String {
        let response = self.http2_client.get(format!("https://127.0.0.1:{}/riotclient/machine-id", self.config.port))
            .basic_auth(&self.config.name, Some(&self.config.token))
            .send()
            .unwrap()
            .text()
            .unwrap()
            .replace("\"","");

        return response;
    }




}