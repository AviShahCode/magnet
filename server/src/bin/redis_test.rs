use redis;
use redis::TypedCommands;

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    // println!("{:?}", con.get("key1"));
    println!("{:?}", con.get("key77"));

    con.set("key1", "value".repeat(100_000_000)).unwrap();
    con.flushdb().unwrap();
}
