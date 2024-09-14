pub mod auth_service;
pub mod session;
pub mod store;

pub struct Services {
    service: Vec<Box<dyn Service>>,
}

#[allow(dead_code)]
impl Services {
    pub fn new() -> Services {
        Services {
            service: Vec::new(),
        }
    }
    pub fn add_service(&mut self, service: Box<dyn Service>) {
        self.service.push(service);
    }
    pub fn get_services(&self) -> &Vec<Box<dyn Service>> {
        &self.service
    }
}

#[allow(dead_code)]
pub trait Service {
    fn get_name(&self) -> &str;
}
