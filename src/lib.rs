use extism_pdk::{host_fn, plugin_fn, FnResult};

#[host_fn("extism-plugin")]
extern "ExtismHost" {
    fn say_bye() -> String;
}

#[plugin_fn]
pub fn greet(name: String) -> FnResult<String> {
    Ok(format!("Hello, {}!", name))
}
