#![allow(dead_code)]

use types::{login, project};
use ureq::Agent;

mod types;

struct App {
    agent: Agent,
    access_token: String,
    employee_id: u64,
}

impl App {
    fn new() -> Self {
        Self {
            agent: Agent::config_builder().build().into(),
            access_token: String::new(),
            employee_id: 0,
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
                "query": "query { me { id } }"
            }))?
            .body_mut()
            .read_json::<serde_json::Value>()?;

        self._set_employee_id(response["data"]["me"]["id"].as_u64().unwrap());
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

    fn _set_access_token(&mut self, access_token: String) {
        self.access_token = access_token;
    }

    fn _set_employee_id(&mut self, employee_id: u64) {
        self.employee_id = employee_id;
    }
}

fn main() {
    let email = "";
    let password = "";
    let mut app = App::new();
    let login = app.login(email, password).unwrap();
    println!("{:?}", login);
}
