use afire::{Content, Server};
use serde_json::json;

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.route(afire::Method::GET, "/api/top-sites", |ctx| {
        let app = ctx.app();
        let top_sites = app.analytics.top_sites(10);

        ctx.content(Content::JSON).text(json!(top_sites?)).send()?;
        Ok(())
    });
}
