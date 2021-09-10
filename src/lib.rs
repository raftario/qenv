use std::{env, path::Path};

const PATH: &str = "/sdcard/Android/data/com.beatgames.beatsaber/files/.env";

#[no_mangle]
pub extern "C" fn setup() {
    tracing_android::init(env!("CARGO_PKG_NAME"));

    if !Path::new(PATH).exists() {
        tracing::info!("no .env file");
        return;
    }

    #[allow(deprecated)]
    let env = match dotenv::from_path_iter(PATH) {
        Ok(env) => env,
        Err(err) => {
            tracing::error!("error reading .env file: {}", err);
            return;
        }
    };

    let env = env.filter_map(|res| match res {
        Ok((k, v)) => Some((k, v)),
        Err(err) => {
            tracing::error!("error reading .env var: {}", err);
            None
        }
    });

    for (k, v) in env {
        tracing::info!("{} = {}", k, v);
        env::set_var(k, v);
    }
}
