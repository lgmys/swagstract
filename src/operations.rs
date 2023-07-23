use anyhow::Result;
use serde_yaml::Value;

use crate::{refs::find_refs, spec::Spec};

#[derive(Debug, Clone)]
pub struct Operation {
    pub path: String,
    pub method: String,
    pub operation_id: String,
    pub refs: Vec<String>,
    pub value: Value,
}

pub fn get_operations(spec: &Spec) -> Result<Vec<Operation>> {
    let paths = spec.paths.clone();

    let mut operations: Vec<Operation> = vec![];

    paths.into_iter().for_each(|(path, methods)| {
        methods.into_iter().for_each(|(method, value)| {
            let operation_id = value["operationId"]
                .as_str()
                .unwrap_or_default()
                .to_string();

            operations.push(Operation {
                path: path.clone(),
                method: method.clone(),
                operation_id,
                value: value.clone(),
                refs: find_refs(serde_yaml::to_string(&value).unwrap_or_default().as_str()),
            });
        });
    });

    Ok(operations)
}

pub fn filter_operations_by_ids(
    operations: &Vec<Operation>,
    operation_ids: &Vec<String>,
) -> Vec<Operation> {
    operations
        .iter()
        .filter(|operation| {
            operation_ids
                .iter()
                .any(|wanted_operation| wanted_operation == &operation.operation_id)
        })
        .cloned()
        .collect()
}
