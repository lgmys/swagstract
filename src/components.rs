use std::{cell::RefCell, collections::HashMap};

use serde_yaml::Value;

use crate::{operations::Operation, spec::Spec};

#[derive(Debug, Clone)]
pub struct ComponentWithChildren {
    pub name: String,
    pub category: String,
    pub path: String,
    pub value: Value,
    pub children: Vec<RefCell<ComponentWithChildren>>,
    pub refs: Vec<String>,
}

#[derive(Debug)]
pub struct ComponentTree {
    pub components: HashMap<String, RefCell<ComponentWithChildren>>,
}

fn flatten_component_with_children(
    component: &ComponentWithChildren,
    flattened: &mut Vec<ComponentWithChildren>,
) {
    flattened.push(component.clone());

    component.children.iter().for_each(|child| {
        flatten_component_with_children(&child.borrow(), flattened);
    });
}

impl ComponentTree {
    pub fn from_spec(spec: &Spec) -> Self {
        let components = spec.components.clone();

        let mut component_tree: HashMap<String, RefCell<ComponentWithChildren>> = HashMap::new();

        components.into_iter().for_each(|(category, members)| {
            members.into_iter().for_each(|(name, value)| {
                let path: String = format!("#/components/{}/{}", category, name);

                if let Ok(value_as_string) = serde_yaml::to_string(&value) {
                    let refs = crate::refs::find_refs(value_as_string.as_str());

                    component_tree.entry(path.clone()).or_insert(RefCell::new(
                        ComponentWithChildren {
                            path,
                            name: name.clone(),
                            category: category.clone(),
                            value,
                            children: vec![],
                            refs,
                        },
                    ));
                }
            });
        });

        component_tree.iter().for_each(|(_name, component)| {
            let children = component
                .borrow()
                .refs
                .iter()
                .map(|ref_name| component_tree.get(ref_name).unwrap().clone())
                .collect();

            component.borrow_mut().children = children;
        });

        Self {
            components: component_tree,
        }
    }

    pub fn find_components_matching(
        &self,
        operations: &Vec<Operation>,
    ) -> Vec<ComponentWithChildren> {
        let mut flattened: Vec<ComponentWithChildren> = vec![];

        self.components
            .iter()
            .filter(|component| {
                operations
                    .iter()
                    .any(|operation| operation.refs.contains(&component.0))
            })
            .for_each(|(_name, component)| {
                flatten_component_with_children(&component.borrow(), &mut flattened)
            });

        flattened
    }
}
