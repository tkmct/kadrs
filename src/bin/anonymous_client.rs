use {
    clap::{App, Arg, ArgMatches, SubCommand},
    kadrs::rpc::Rpc,
    std::net::SocketAddrV4,
};

fn parse_method(matches: ArgMatches) -> Result<Rpc, ()> {
    if let Some(_) = matches.subcommand_matches("ping") {
        return Ok(Rpc::Ping);
    }

    if let Some(sub_match) = matches.subcommand_matches("find_value") {
        let key = sub_match.value_of("key").ok_or(())?;
        return Ok(Rpc::FindValue(key.into()));
    }

    if let Some(sub_match) = matches.subcommand_matches("find_node") {
        let id = sub_match.value_of("id").ok_or(())?;
        return Ok(Rpc::FindNode(id.into()));
    }

    if let Some(sub_match) = matches.subcommand_matches("store") {
        let key = sub_match.value_of("key").ok_or(())?;
        let value = sub_match.value_of("value").ok_or(())?;
        return Ok(Rpc::Store(key.into(), value.into()));
    }

    Err(())
}

#[async_std::main]
async fn main() -> Result<(), ()> {
    let app = App::new("kadrs-client")
        .version("0.1.0")
        .about("client app for kadrs")
        .arg(Arg::with_name("host").required(true))
        .subcommands(vec![
            SubCommand::with_name("ping").about("PING to check if node with given host is alive"),
            SubCommand::with_name("find_value")
                .about("FIND_VALUE to get stored value")
                .arg(Arg::with_name("key").required(true)),
            SubCommand::with_name("find_node")
                .about("FIND_NODE to get host of given id")
                .arg(Arg::with_name("id").required(true)),
            SubCommand::with_name("store")
                .about("STORE given key value pair")
                .args(&vec![
                    Arg::with_name("key").required(true),
                    Arg::with_name("value").required(true),
                ]),
        ]);
    let matches = app.get_matches();
    let host: SocketAddrV4 = match matches.value_of("host").unwrap().parse() {
        Ok(addr) => addr,
        Err(_) => panic!("Invalid host string"),
    };
    let rpc = parse_method(matches)?;
    println!("request to host: {:?} {:?}", host, rpc);

    Ok(())
}
