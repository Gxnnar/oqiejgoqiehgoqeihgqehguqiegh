use afire::{extensions::ServeStatic, Middleware, Server};
use app::App;

mod analytics;
mod app;
mod config;
mod proxy;

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
        .state(app);

    ServeStatic::new("./web/static").attach(&mut server);
    proxy::attach(&mut server);

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
// - Analytis in a sqlite db and graphs on the homepage
// - Said homepage will be a search like box with a grayed out `https://`
//   For the user to type an address into, if its not a valid address, ddg search it.
