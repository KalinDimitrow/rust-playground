use libswserver::SimpleWebServer;

fn main() {
    match  SimpleWebServer::new("127.0.0.1:7878", 5) {
        Ok(server) => {
            server.sync_start();
        },
        Err(error) => {
            println!("{}", error);
        }
    };
}

