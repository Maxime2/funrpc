use actix_web::{actix, client, HttpMessage};
use structopt::StructOpt;
#[macro_use]
extern crate serde_derive;
use futures::future::Future;
use serde_json;

#[derive(StructOpt, Debug)]
#[structopt(name = "funrpc")]
/// Lachesis node CLI interface
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// node API hostname/address
    #[structopt(short = "h", long = "host", default_value = "localhost")]
    hostname: String,

    /// Node API port number
    #[structopt(short = "p", long = "port", default_value = "8080")]
    port: i32,

    /// Command
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "cmd", help = "subcommands")]
enum Command {
    #[structopt(name = "accounts")]
    /// get controlled accounts
    Accounts,

    #[structopt(name = "account")]
    /// get any account
    Account {
        #[structopt(short = "n", long = "number")]
        number: String,
    },
    #[structopt(name = "tx")]
    /// sends transaction or get transaction status
    Tx(Tx),

    #[structopt(name = "rawtx")]
    /// sends raw signed transaction
    RawTx {
        #[structopt(short = "d", help = "raw transaction")]
        doc: String,
    },
}

#[derive(StructOpt, Debug, Serialize)]
struct TxD {
    #[structopt(short = "f", long = "from", help = "account from")]
    from: String,
    #[structopt(short = "t", long = "to", help = "account to")]
    to: String,
    #[structopt(short = "v", long = "value", help = "value in transaction")]
    value: String,
}

#[derive(StructOpt, Debug)]
enum Tx {
    #[structopt(name = "d")]
    /// Sends transaction JSON
    #[structopt(name = "d")]
    D(TxD),
    /// request status of transaction
    #[structopt(name = "h")]
    H {
        /// Hexadecimal transaction hash
        tx_hash: String,
    },
}

fn main() {
    let opt = Opt::from_args();
    let mut url = String::new();
    let mut body = "".to_string();
    let mut post: bool = false;
    //println!("{:?}", opt);
    if opt.verbose > 0 {
        if opt.debug {
            println!("Debug mode is enabled");
        }
        println!("Verbosity level is {}", opt.verbose);
    }
    match opt.cmd {
        Command::Accounts => {
            url = format!("http://{}:{}/accounts", opt.hostname, opt.port);
            if opt.verbose > 0 {
                println!("Controlled accounts from {}:{}", opt.hostname, opt.port);
            }
        }
        Command::Account { number } => {
            url = format!("http://{}:{}/account/{}", opt.hostname, opt.port, number);
            if opt.verbose > 0 {
                println!("Account from {}:{}", opt.hostname, opt.port);
            }
        }
        Command::Tx(tx) => {
            url = format!("http://{}:{}/tx", opt.hostname, opt.port);
            match tx {
                Tx::H { tx_hash } => {
                    url = format!("{}/{}", url, tx_hash);
                }
                Tx::D(tx_d) => {
                    post = true;
                    body = serde_json::to_string(&tx_d).unwrap();
                }
            }
        }
        Command::RawTx { doc } => {
            post = true;
            body = doc;
            url = format!("http://{}:{}/rawtx", opt.hostname, opt.port);
            if opt.verbose > 0 {
                println!(
                    "Sending raw signed transaction to {}:{}",
                    opt.hostname, opt.port
                );
            }
        }
    }

    actix::run(|| {
        match post {
            false => client::ClientRequest::get(url),
            true => client::ClientRequest::post(url),
        }
        .header("User-Agent", "Actix-web")
        .body(actix_web::Body::from(body))
        .unwrap()
        .send()
        .map_err(|_| ())
        .and_then(|response| {
            // <- server http response
            match response.body().wait() {
                Ok(body) => {
                    let _foo: String = String::from_utf8(body.to_vec()).unwrap();
                    println!("{}", _foo);
                    actix::System::current().stop();
                    Ok(())
                }
                Err(_err) => {
                    println!("error {:?}", _err);
                    actix::System::current().stop();
                    Err(())
                }
            }
        })
    });
}
