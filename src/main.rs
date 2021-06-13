use openssh::*;

use clap::{App, Arg};

async fn create_screen(name: &str, session: &openssh::Session) {
   let cmd = session.command("screen")
                    .args(&["-S", name, "-d", "-m"])
                    .status()
                    .await.unwrap();
    eprintln!("Status: {}", cmd);
}

async fn exec_cmd_screen(name: &str, session: &openssh::Session, cmd: String) {
   let cmd = session.command("screen")
                    .args(&["-r", name, "-X", "stuff", &cmd])
                    .status()
                    .await.unwrap();
    eprintln!("Status: {}", cmd);
}

#[tokio::main]
async fn main() {

    let matches = App::new("poc-rscreen")
        .arg(Arg::new("server")
             .short('s')
             .long("server")
             .about("Sets the targeted server hostname")
             .takes_value(true)
             .required(true))
        .arg(Arg::new("cmd")
             .short('c')
             .long("cmd")
             .about("Command to execute in screen")
             .takes_value(true)
             .required(true))
        .arg(Arg::new("name")
             .short('n')
             .long("name")
             .about("Screen session name")
             .takes_value(true)
             .default_value("rusty"))
        .get_matches();


    let server = matches.value_of("server").unwrap();
    let session = Session::connect(server, KnownHosts::Strict).await.unwrap();

    let name = matches.value_of("name").unwrap();
    create_screen(name, &session).await;

    let cmd = matches.value_of("cmd").unwrap();
    let command = format!("{}\n", cmd);
    exec_cmd_screen(name, &session, command).await;

    session.close().await.unwrap();
}
