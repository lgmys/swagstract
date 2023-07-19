use crate::spec::{raw_from_spec, Spec};

pub fn print(spec: &Spec) {
    let raw_spec = raw_from_spec(spec);

    print!("{}", serde_yaml::to_string(&raw_spec).unwrap())
}
