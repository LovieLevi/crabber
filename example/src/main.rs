use crabber::{ServeType, Server, SpecialServeType};

fn main() {
    let mut server: Server = Server::new(8080);
    server.serve("/", ServeType::File, "static/index.html");
    server.serve("/info", ServeType::File, "static/info.html");
    server.special_serve(SpecialServeType::FourOhFour, "static/404.html");
    server.run();
}
