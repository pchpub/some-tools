use std::{fs::File, io::Write, sync::{Mutex, Arc}};

use whois_rust::{WhoIsLookupOptions, WhoIs};

fn main() {
    let mut file = File::create("rs_domain.log").unwrap();
    let whois = WhoIs::from_path("servers.json").unwrap();
    let mut domains = Vec::with_capacity(36 * 36);
    let mut chars = Vec::new();
    // for i in 48..58 {
    //     chars.push(char::from_u32(i).unwrap());
    // }
    // for i in 97..123 {
    //     chars.push(char::from_u32(i).unwrap());
    // }
    chars = vec!['a','w','e','r','s','z','x','c','n','m'];
    for i in chars.iter() {
        for j in chars.iter() {
            domains.push(format!("{}{}.rs", i, j));
        }
    }
    // let domain_mutex = Mutex::new(Arc::new(domains));
    // for i in 0..10{
    //     std::thread::spawn(||{

    //     });
    // }
    for domain in domains.iter() {
        let rsp_data = if let Ok(value) = whois.lookup(WhoIsLookupOptions::from_string(domain).unwrap()){
            value
        }else{
            if let Ok(value) = whois.lookup(WhoIsLookupOptions::from_string(domain).unwrap()) {
                value
            }else{
                println!("{} failed", domain);
                "".to_owned()
            }
        };

        // println!("{}", rsp_data);
        // let foo: serde_json::Value = serde_json::from_str(&rsp_data).unwrap();
        // let object = foo.as_object().unwrap();
        if !rsp_data.contains("Expiration date") {
            println!("{} ", domain);
            println!("{}", rsp_data);
            file.write(format!("{}\n", domain).as_bytes()).unwrap();
            // file.write(
            //     format!(
            //         "Expiration date: {}\n",
            //         object
            //             .get("Expiration date")
            //             .unwrap_or(&serde_json::Value::String("HHHHHHHHHHHHHHHH".to_string()))
            //             .as_str()
            //             .unwrap()
            //     )
            //     .as_bytes(),
            // )
            // .unwrap();
        }else{
            
        }
        
    }
}
