use std::{fs::File, io::{Write, Read}};
use serde_json::{Value, json};
use rand;

pub struct Data {
	data: Value
}

impl Data {
	pub fn new() -> Self {
		let mut result = String::new();

		match File::open("data.json") {
			Ok(mut file) => {file.read_to_string(&mut result).unwrap();},
			Err(_) => {
				if let Ok(mut f) = File::create("data.json") {
					f.write_all(json!({"projects": []}).to_string().as_bytes()).unwrap();
					f.flush().unwrap();

					result = json!({"projects": []}).to_string();
				};
			}
		}

		Self {
			data: serde_json::from_str(result.as_str()).unwrap()
		}
	}

	fn rewrite(&self) {
		if let Ok(mut file) = File::create("data.json") {
			file.write_all(self.data.to_string().as_bytes()).unwrap();
			file.flush().unwrap();
		}
	}
}

impl Data {
	pub fn get(&self, a: &str, pid: &str, tid: &str) -> String {
		let mut result = self.data.to_string();

		if a == "project" {
			for project in self.data["projects"].as_array().unwrap() {
				if project["id"] == pid {
					result = project.to_string();
					break;
				}
			}

		} else if a == "task" {
			for project in self.data["projects"].as_array().unwrap() {
				if project["id"] == pid {
					for task in project["tasks"].as_array().unwrap() {
						if task["id"] == tid {
							result = task.to_string();
						break;
						}
					}
					break;
				}
			}
		}

		result
	}

	pub fn create(&mut self, a: &str, pid: &str) -> String {
		let smth = if a == "project" { // project
			json!({
				"id": rand::random::<u32>().to_string().as_str(),
				"title": "New Project",
				"description": "Project description..",
				"count": "0/0",
				"tasks": []
			})
		} else { // task
			json!({
				"id": rand::random::<u32>().to_string().as_str(),
				"title": "Task",
				"description": "Task description..",
				"done": false
			})
		};


		if a == "project" {
			self.data["projects"].as_array_mut().unwrap().push(smth.clone());
		} else if a == "task" {
			for project in self.data["projects"].as_array_mut().unwrap() {
				if project["id"] == pid {
					project["tasks"].as_array_mut().unwrap().push(smth.clone());
					break;
				}
			}
		}


		self.rewrite();
		smth.to_string()
	}

	pub fn update(&mut self, a: &str, pid: &str, tid: &str, param: &str, value: &str) {
		if a == "project" {
			for project in self.data["projects"].as_array_mut().unwrap() {
				if project["id"] == pid {
					project[param] = value.into();
					break;
				}
			}

		} else if a == "task" {
			for project in self.data["projects"].as_array_mut().unwrap() {
				if project["id"] == pid {
					for task in project["tasks"].as_array_mut().unwrap() {
						if task["id"] == tid {
							task[param] = if param == "done" {
								if value == "true" {
									json!(true)
								} else {
									json!(false)
								}
							} else {
								value.into()
							};
							break;
						}
					}
					break;
				}
			}
		}

		self.rewrite();
	}

	pub fn delete(&mut self, a: &str, pid: &str, tid: &str) {
		if a == "project" {
			let projects = self.data["projects"].as_array_mut().unwrap();

			for i in 0..(projects.len()) {
				if projects[i]["id"] == pid {
					projects.remove(i);
					break;
				}
			}

		} else if a == "task" {
			for project in self.data["projects"].as_array_mut().unwrap() {
				if project["id"] == pid {
					let tasks = project["tasks"].as_array_mut().unwrap();

					for i in 0..(tasks.len()) {
						if tasks[i]["id"] == tid {
							tasks.remove(i);
							break;
						}
					}
					break;
				}
			}
		}

		self.rewrite();
	}
}