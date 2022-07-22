use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};
use std::{env, time::Duration};

mod tricoder_error;
pub use tricoder_error::Error;
mod tricoder_model;
mod tricoder_ports;
mod tricoder_subdomains;
use tricoder_model::Subdomain;
mod tricoder_common_ports;

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage.into());
    }

    let target = args[1].as_str();

    let http_timeout = Duration::from_secs(15);
    let http_client = Client::builder()
//        .redirect(redirect::Policy::limited(14))
//        .timeout(http_timeout)
        .build()?;

    // we use a custom threadpool to improve speed
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();


    println!("iiiiiiiiiiiiiiiiiiiii 0");
    // pool.install is required to use our custom threadpool, instead of rayon's default
    pool.install(|| {
        println!("--------1 )))))))))))))))))");
        let scan_result: Vec<Subdomain> = dbg!(tricoder_subdomains::enumerate(&http_client, target)
            .unwrap())
            .into_iter()
            .map(tricoder_ports::scan_ports)
            .collect();

        println!("---------2 ))))))))))))))))))");
        for subdomain in scan_result {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("  {}", port.port);
            }
        }
        println!("--------------- asdfasdfas )))))))))))))))))))");
    });
    println!("kkkkkkkkkkkkkkk final");
    Ok(())
}
