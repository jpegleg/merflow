use postgres::{Client, Error, NoTls};
use redis::Commands;
use chrono::prelude::*;
use std::{thread, time, env};

extern crate redis;

fn redisset(insertit: String, valit: String) -> redis::RedisResult<()> {
    let redis_client = redis::Client::open("redis://localhost:6379/")?;
    let mut rcon = redis_client.get_connection()?;
    let _seasnails: String = rcon.set(insertit, valit).unwrap();
    Ok(())
}

fn main() -> Result<(), Error> {
  loop {
    let consetcred: String = env::var("pcred").unwrap();
    let mut client = Client::connect(
          &consetcred,
        NoTls,
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            password        VARCHAR NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
            )
    ",
    )?;

    for row in client.query("SELECT id, username, password, email FROM users", &[])? {
        let id: i32 = row.get(0);
        let username: &str = row.get(1);
        let password: &str = row.get(2);
        let email: &str = row.get(3);
        let hydrox = redisset(id.to_string(), email.to_string());
        let initc = Utc::now();
        match hydrox  {
            Ok(()) => (),
            Err(error) => panic!("{} Problem connecting to redis: {:?}", initc, error),
        };
        let hydron = redisset(username.to_string(), password.to_string());
        let initn = Utc::now();
        match hydron  {
            Ok(()) => (),
            Err(error) => panic!("{} Problem connecting to redis: {:?}", initn, error),
        };

    };

    let millis = time::Duration::from_millis(5987);
    thread::sleep(millis);
  };

}
