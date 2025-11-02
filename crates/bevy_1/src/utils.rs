use std::env;
// see some envs
pub fn see_some_env(env_names: &[&str]) {
    for i in env_names {
        match env::var(i) {
            Ok(val) => {
                println!("{i}= {}", val)
            }
            Err(e) => println!(
                "Couldn't read WGPU_ADAPTER_NAME: {}",
                e
            ),
        }
    }
}
