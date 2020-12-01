use crate::{
    interpreter::Result,
    parser::pair::GenericPair,
    parser::*,
    values::{Procedure, RealNumberInternalTrait, Value},
};

fn display<R: RealNumberInternalTrait>(
    arguments: impl IntoIterator<Item = Value<R>>,
) -> Result<Value<R>> {
    print!("{}", arguments.into_iter().next().unwrap());
    Ok(Value::Void)
}

pub fn library_map<'a, R: RealNumberInternalTrait>() -> Vec<(String, Value<R>)> {
    vec![pure_function_mapping!(
        "display",
        param_fixed!["value"],
        display
    )]
}
