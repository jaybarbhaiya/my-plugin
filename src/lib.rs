use extism_pdk::{host_fn, plugin_fn, FnResult};

#[host_fn("extism-plugin")]
extern "ExtismHost" {
    fn say_bye() -> String;
}

#[plugin_fn]
pub fn greet(name: String) -> FnResult<String> {
    let say_bye = unsafe { say_bye()? };
    Ok(format!("Hello, {}. {}!", name, say_bye))
}
