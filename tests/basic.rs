use std::net::SocketAddr;
use tempdir::TempDir;
use light_letter;

const SITES_CONFIG: &'static str = r#"
    [net]
    ip = "0.0.0.0"
    port = [0]

    [db]
    username = "light_letter"
    password = "light_letter"

    [[site]]
    name = "local-1"
    database = "light_letter_local_1"
    type = "blog"
    host = "localhost:2180"

    [[site]]
    name = "local-2"
    type = "static"
    host = "127.0.0.1:2180"
"#;

fn generate_server<F>(f: F) where F: FnOnce(u16) + Send + 'static {
    let temp_dir = TempDir::new("light-letter-test").unwrap();
    let temp_dir_path = temp_dir.path();
    let config = temp_dir_path.join("config.toml");
    std::fs::write(config, SITES_CONFIG).unwrap();
    let p = std::path::PathBuf::from(temp_dir_path);
    let sites_server = light_letter::SitesServer::new(p);
    sites_server.run_with_thread(move |mut sites_server| {
        let port = match sites_server.addrs()[0] {
            SocketAddr::V4(a) => a.port(),
            SocketAddr::V6(a) => a.port(),
        };
        f(port);
        sites_server.stop(true);
    });
    temp_dir.close().unwrap();
}

#[test]
fn start_simple_server() {
    generate_server(|_port| {
        // empty
    });
}
