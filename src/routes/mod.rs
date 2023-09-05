use afire::Server;

use crate::app::App;

mod top_sites;

pub fn attach(server: &mut Server<App>) {
    top_sites::attach(server);
}
