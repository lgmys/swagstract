use anyhow::Result;
use serde_yaml::Value;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Info {
    pub title: String,
    pub version: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RawOperation {
    #[serde(rename = "operationId")]
    pub operation_id: String,
    pub summary: Option<String>,
    pub parameters: Option<Vec<Value>>,
    pub responses: HashMap<String, Value>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RawSpec {
    pub paths: HashMap<String, HashMap<String, RawOperation>>,
    pub openapi: String,
    pub info: Info,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct Operation {
    pub operation_id: String,
    pub summary: Option<String>,
    pub parameters: Option<Vec<Value>>,
    pub responses: HashMap<String, Value>,
    pub tags: Option<Vec<String>>,
    pub method: String,
    pub path: String,
}

#[derive(Debug)]
pub struct Spec {
    pub operations: Vec<Operation>,
    pub openapi: String,
    pub info: Info,
}

fn spec_from_raw(raw_spec: RawSpec) -> Spec {
    let operations = raw_spec
        .paths
        .into_iter()
        .flat_map(|(path, operations)| {
            operations
                .into_iter()
                .map(|(method, raw_operation)| Operation {
                    operation_id: raw_operation.operation_id,
                    summary: raw_operation.summary,
                    parameters: raw_operation.parameters,
                    responses: raw_operation.responses,
                    tags: raw_operation.tags,
                    method: method.clone(),
                    path: path.clone(),
                })
                .collect::<Vec<Operation>>()
        })
        .collect();

    Spec {
        operations,
        openapi: raw_spec.openapi,
        info: raw_spec.info,
    }
}

pub fn raw_from_spec(spec: &Spec) -> RawSpec {
    let paths = spec
        .operations
        .clone()
        .into_iter()
        .fold(HashMap::new(), |mut acc, operation| {
            let path = acc.entry(operation.path).or_insert(HashMap::new());

            path.insert(
                operation.method,
                RawOperation {
                    operation_id: operation.operation_id,
                    summary: operation.summary,
                    parameters: operation.parameters,
                    responses: operation.responses,
                    tags: operation.tags,
                },
            );

            acc
        });

    RawSpec {
        paths,
        openapi: spec.openapi.clone(),
        info: spec.info.clone(),
    }
}

pub fn from_file(path: &Path) -> Result<Spec> {
    let spec_file = fs::read_to_string(path)?;
    let raw_spec: RawSpec = serde_yaml::from_str(&spec_file)?;
    let spec = spec_from_raw(raw_spec);

    Ok(spec)
}
