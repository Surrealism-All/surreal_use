<img src="https://img.shields.io/badge/surreal_use-0.1.0-orange?style=flat-square&logo=rust&logoColor=%23fff&labelColor=%23DEA584&color=%23DEA584">  <img src="https://img.shields.io/badge/License-MIT-orange?style=flat-square&logoColor=%23fff&labelColor=%2323B898&color=%2323B898">  <img src="https://img.shields.io/badge/For-surrealdb-purple?style=flat-square&logoColor=%239055ED&labelColor=%239055ED&color=%239055ED">

***


# surreal_use

**An extension library based on the Surrealdb library to help users develop more conveniently**

```txt
 ________  ___  ___  ________  ________  _______   ________  ___        ___  ___  ________  _______
|\   ____\|\  \|\  \|\   __  \|\   __  \|\  ___ \ |\   __  \|\  \      |\  \|\  \|\   ____\|\  ___ \
\ \  \___|\ \  \\\  \ \  \|\  \ \  \|\  \ \   __/|\ \  \|\  \ \  \     \ \  \\\  \ \  \___|\ \   __/|
 \ \_____  \ \  \\\  \ \   _  _\ \   _  _\ \  \_|/_\ \   __  \ \  \     \ \  \\\  \ \_____  \ \  \_|/__
  \|____|\  \ \  \\\  \ \  \\  \\ \  \\  \\ \  \_|\ \ \  \ \  \ \  \____ \ \  \\\  \|____|\  \ \  \_|\ \
    ____\_\  \ \_______\ \__\\ _\\ \__\\ _\\ \_______\ \__\ \__\ \_______\\ \_______\____\_\  \ \_______\
   |\_________\|_______|\|__|\|__|\|__|\|__|\|_______|\|__|\|__|\|_______| \|_______|\_________\|_______|
   \|_________|                                                                     \|_________|
```

- author：syf20020816@outlook.com
- createDate：20240115
- updateDate：20240127
- version：0.1.0
- email：syf20020816@outlook.com

## What surreal_use do

- Detaching database configurations and code
- Reduce manual writing of SurrealQL statements
- Perform differentiated API queries
- Effortlessly perform complex queries
- Enable users to feel seamless integration with the Surrealdb library

## QuickStart
### write surrealdb.config.json
```json
{
  "endpoint":"127.0.0.1",
  "port":10086,
  "auth":{
    "user":"root",
    "pass":"root"
  }
}
```
### use surreal_use
``` rust
use lazy_static::lazy_static;
use surreal_use::{
    config::{auth::Root, parser::Parsers, AuthBridger},
    core::Stmt,
};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

// use lazy static macro
lazy_static! {
    static ref DB: Surreal<Client> = Surreal::init();
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Using seasonal_ Use to obtain the configuration of surrealdbunconfig.json under the project package
    let config = Parsers::Json.parse_to_config(None);
    DB.connect::<Ws>(config.url()).await?;
    // transfer to credential Root
    let credentail: Root = config.get_auth().into();
    // Sigin use Root
    // Return Jwt struct
    let _ = DB.signin(credentail.to_lower_cast()).await?;
    let _ = DB.use_ns("test").use_db("test").await?;
    let select = Stmt::select().table("user".into()).field_all().to_string();
    let query = DB.query(&select).await?;
    dbg!(query);
    Ok(())
}
```
## Attation
There are many structures in use with the same name as the surrealdb library,
which exist as extensions to the source library

## Features

- [x] select
- [x] update
- [x] insert
- [x] delete
- [x] create
- [x] use
- [ ] begin
- [ ] break
- [ ] cancel
- [ ] commit
- [ ] continue
- [ ] define
- [ ] for
- [ ] if
- [ ] info
- [ ] kill
- [ ] let
- [ ] live select
- [ ] relate
- [ ] remove
- [ ] return
- [ ] show
- [ ] sleep
- [ ] throw