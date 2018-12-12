extern crate structopt;
extern crate elefren;

use std::error::Error;

use structopt::StructOpt;
use elefren::prelude::*;
use elefren::helpers::cli;

#[derive(StructOpt)]
struct Options {
    domain: String,
    #[structopt(long = "follow-new")]
    follow: bool,
    #[structopt(long = "unfollow-old")]
    unfollow: bool,
}

fn main() -> Result<(), Box<Error>> {
    let opts = Options::from_args();
    let domain = format!("https://{}", &opts.domain);
    let registration = Registration::new(domain)
            .client_name("find-moved-followers")
            .scopes(Scopes::read_all() | Scopes::follow() | Scopes::write_all())
            .build()?;
    let client = cli::authenticate(registration)?;
    let me = client.verify_credentials()?;

    let following = client.following(&me.id)?.items_iter();

    for account in following.filter(|f| f.moved.is_some()) {
        if let Some(to) = account.moved {
            println!("moved from {} to {}", account.acct, to.acct);
            if opts.unfollow {
                if let Err(e) = client.unfollow(&account.id) {
                    eprintln!("Error unfollowing {}: {:?}", account.acct, e.description());
                }
            }

            if opts.follow {
                if let Err(e) = client.follow(&to.id) {
                    eprintln!("Error following {}: {:?}", to.acct, e.description());
                }
            }
        }
    }
    Ok(())
}
