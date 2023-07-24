use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::{components::ComponentWithChildren, operations::Operation};

#[derive(Deserialize, Serialize, Debug)]
pub struct Spec {
    pub components: HashMap<String, HashMap<String, Value>>,
    pub paths: HashMap<String, HashMap<String, Value>>,
    pub openapi: Option<String>,
    pub info: Option<Value>,
    pub servers: Option<Value>,
}

#[derive(Default)]
pub struct SpecBuilder {
    components: Vec<ComponentWithChildren>,
    operations: Vec<Operation>,
    info: Option<Value>,
    servers: Option<Value>,
    openapi: Option<String>,
}

impl SpecBuilder {
    pub fn set_components(mut self, components: Vec<ComponentWithChildren>) -> Self {
        self.components = components;
        self
    }

    pub fn set_operations(mut self, operations: Vec<Operation>) -> Self {
        self.operations = operations;
        self
    }

    pub fn set_info(mut self, info: Option<Value>) -> Self {
        self.info = info;
        self
    }

    pub fn set_servers(mut self, servers: Option<Value>) -> Self {
        self.servers = servers;
        self
    }

    pub fn set_openapi(mut self, openapi: Option<String>) -> Self {
        self.openapi = openapi;
        self
    }

    pub fn build(self) -> Spec {
        let mut components: HashMap<String, HashMap<String, Value>> = HashMap::new();
        let mut paths: HashMap<String, HashMap<String, Value>> = HashMap::new();

        self.components.iter().for_each(|component| {
            let category = component.category.clone();
            let name = component.name.clone();
            let value = component.value.clone();

            components
                .entry(category)
                .or_insert(HashMap::new())
                .insert(name, value);
        });

        self.operations.iter().for_each(|operation| {
            let path = operation.path.clone();
            let method = operation.method.clone();
            let value = operation.value.clone();

            paths
                .entry(path)
                .or_insert(HashMap::new())
                .insert(method, value);
        });

        Spec {
            components,
            paths,
            openapi: self.openapi,
            info: self.info,
            servers: self.servers,
        }
    }
}
