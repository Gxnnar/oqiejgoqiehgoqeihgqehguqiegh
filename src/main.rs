use afire::{extensions::ServeStatic, Middleware, Server};
use app::App;

mod analytics;
mod app;
mod config;
mod proxy;
mod routes;

const BLOCKED_HEADERS: &[&str] = &[
    "transfer-encoding",
    "connection",
    "content-security-policy",
    "referrer-policy",
];

fn main() -> anyhow::Result<()> {
    let app = App::new("./config.toml".into())?;
    let mut server = Server::new(app.config.host, app.config.port)
        .workers(10)
        .keep_alive(false)
        .state(app);

    ServeStatic::new("./web").attach(&mut server);
    proxy::attach(&mut server);
    routes::attach(&mut server);

    let exit_app = server.app();
    ctrlc::set_handler(move || {
        println!("[*] Shutting Down...");
        exit_app.analytics.cleanup().unwrap();
        std::process::exit(0);
    })?;

    server.run()?;
    Ok(())
}

// == TODOS ==
// - some kinda caching mechanism
// - Finish top-sites with https://paste.connorcode.com/b/2ef343f4-5681-4c37-863e-490a6cfe8c27
