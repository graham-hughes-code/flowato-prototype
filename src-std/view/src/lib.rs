use extism_pdk::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Input {
    pub a: ContextValue
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum ContextValue {
    S(String),
    N(f64)
}

#[derive(Serialize)]
struct Output {
    pub context: ContextValue
}

#[plugin_fn]
pub fn view(input: String) -> FnResult<Json<Output>> {
    let input_data: Input;

    match serde_json::from_str(&input) {
        Ok(data) => {
            input_data = data;
        },
        Err(err) => return Err(WithReturnCode::from(err))
    }

    Ok(Json(Output { context: input_data.a }))
}


#[plugin_fn]
pub fn describe_node(_: ()) -> FnResult<String>
{
    let dis: String =
        r#"{"inputs": {
                "a": {
                    "type": ["Number", "String"]
                }
            }, 
            "Output": {
                "context": {
                    "type": ["Number", "String"]
                }
            }
        }"#.to_string();

    Ok(dis)
}

#[plugin_fn]
pub fn node_frontend(_: ()) -> FnResult<&'static str>
{
    let web_comp: &'static str = include_str!("view.js");
    Ok(web_comp)
}