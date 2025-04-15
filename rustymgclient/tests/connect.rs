use rustymgclient::MgConnection;

#[test]
fn test_connect_to_memgraph() {
    let result = MgConnection::connect("127.0.0.1:7687");
    assert!(result.is_ok());

    if let Ok(conn) = result {
        println!("Connected using Bolt v{}", conn.bolt_version);
        assert_ne!(conn.bolt_version, 0);
    }
}