use clap::{App, Arg, SubCommand};

#[async_std::main]
async fn main() {
    let app = App::new("kadrs-client")
        .version("0.1.0")
        .about("client app for kadrs")
        .subcommands(vec![
            SubCommand::with_name("ping").about("PING to check if node with given host is alive"),
            SubCommand::with_name("find_value")
                .about("FIND_VALUE to get stored value")
                .args(&vec![
                    Arg::with_name("host").help("host name. ex) 127.0.0.1:8080"),
                    Arg::with_name("key"),
                ]),
            SubCommand::with_name("find_node")
                .about("FIND_NODE to get host of given id")
                .arg(Arg::with_name("id")),
            SubCommand::with_name("store")
                .about("STORE given key value pair")
                .args(&vec![Arg::with_name("key"), Arg::with_name("value")]),
        ]);
    let matches = app.get_matches();
    println!("{:?}", matches);
}
