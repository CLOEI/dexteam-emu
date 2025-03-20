#![allow(dead_code)]

use std::fs;

use types::{login, project};
use ureq::Agent;

mod types;

#[derive(serde::Deserialize, serde::Serialize)]
struct Config {
    email: String,
    password: String,
}

struct App {
    agent: Agent,
    access_token: String,
    employee_id: u64,
    name: String,
}

impl App {
    fn new() -> Self {
        Self {
            agent: Agent::config_builder().build().into(),
            access_token: String::new(),
            employee_id: 0,
            name: String::new(),
        }
    }

    fn login(&mut self, email: &str, password: &str) -> Result<login::LoginResponse, ureq::Error> {
        let response = self.agent
            .post("https://login.dextion.com/email_login")
            .send_json(serde_json::json!({
                "email": email,
                "password": password,
            }))?
            .body_mut()
            .read_json::<login::LoginResponse>()?;

        self._set_access_token(response.access_token.clone());
        self.get_id().unwrap();
        Ok(response)
    }

    fn get_id(&mut self) -> Result<u64, ureq::Error> {
        if self.employee_id != 0 {
            return Ok(self.employee_id);
        }

        let response = self.agent
            .post("https://ql.dextion.com/query")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("accept-version", "v4")
            .header("Accept-Encoding", "gzip")
            .send_json(serde_json::json!({
                "query": "query { me { id name } }"
            }))?
            .body_mut()
            .read_json::<serde_json::Value>()?;

        self._set_employee_id(response["data"]["me"]["id"].as_u64().unwrap());
        self._set_name(response["data"]["me"]["name"].as_str().unwrap().to_string());
        Ok(self.employee_id)
    }

    fn get_projects(&mut self) -> Result<project::ProjectsResponse, ureq::Error> {
        let response = self.agent
            .post("https://ql.dextion.com/query")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("accept-version", "v4")
            .header("Accept-Encoding", "gzip")
            .send_json(serde_json::json!({
                "query": "query GetProjectList { projects { id project_name self { id project_id staff { id employee { outlet_id id } } } } }"
            }))?
            .body_mut()
            .read_json::<project::ProjectsResponse>()?;

        Ok(response)
    }

    fn checkin(&self) -> Result<(), ureq::Error> {
        let response = self.agent
            .post("https://ql.dextion.com/query")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("accept-version", "v4")
            .header("Accept-Encoding", "gzip")
            .send_json(serde_json::json!({
                "operationName": "checkIn",
                "variables": {
                    "timezone": "+07:00",
                    "employeeId": 2511,
                    "macAddressIn": "00:13:10:85:fe:01",
                    "outletId": 634,
                    "lat": "37.4219983",
                    "lng": "-122.084"
                },
                "query": "mutation checkIn($timezone: String!, $employeeId: Int!, $macAddressIn: String!, $outletId: Int!, $lat: String!, $lng: String!, $in_photo_url: String) { attendance { check_in(input: { user_timezone: $timezone employee_id: $employeeId mac_addr_in: $macAddressIn latitude_in: $lat longitude_in: $lng in_photo_url: $in_photo_url } , outlet_id: $outletId) { id in mac_addr_in user_timezone employee_id latitude_in longitude_in } } }"
            }))?
            .body_mut()
            .read_json::<serde_json::Value>()?;

        if response.get("errors").is_some() {
            let message = response["errors"][0]["message"].as_str().unwrap();
            println!("{}", message);
        } else {
            println!("Checkin success");
        }

        Ok(())
    }

    fn _set_access_token(&mut self, access_token: String) {
        self.access_token = access_token;
    }

    fn _set_employee_id(&mut self, employee_id: u64) {
        self.employee_id = employee_id;
    }

    fn _set_name(&mut self, name: String) {
        self.name = name;
    }
}

fn main() {
    if !std::path::Path::new("config.json").exists() {
        let config = Config {
            email: String::from(""),
            password: String::from(""),
        };

        fs::write("config.json", serde_json::to_string_pretty(&config).unwrap()).unwrap();
        println!("Please fill in your email and password in config.json");
    } else {
        let config: Config = serde_json::from_str(&fs::read_to_string("config.json").unwrap()).unwrap();
        let mut app = App::new();
        app.login(&config.email, &config.password).unwrap();
        println!("Logged in as {}({})", app.name, app.employee_id);
        app.checkin().unwrap();
    }
}
