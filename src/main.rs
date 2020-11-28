// (my app for myself us as daily works)
extern crate clap;
extern crate dotenv;
extern crate urlencoding;
extern crate serde_json;

use std::env;
use dotenv::dotenv;
use clap::{Arg, App, SubCommand, ArgMatches};


fn main() {
    // load env
    dotenv().ok();

    let matches = App::new("My Super Program")
        .version("1.0")
        .author("xiaoyu xu. <xuxy@example.com>")
        .about("Does awesome things")
        .subcommand(SubCommand::with_name("ip")
                    .about("query ip for location")
                    .version("1.0")
                    .author("xiaoyu xu. <xuxy@example.com>")
                    .arg(Arg::with_name("ADDRESS")
                         .help("input standard ip output the location of ip")
                         .required(true)
                         .index(1)
                    ))
        .get_matches();

    
    match executor_factory(&matches) {
        Some(executor) => executor.exe(),
        None => (),            
    };

}


trait Executor {
    fn exe(&self);
}


fn executor_factory(matches: &ArgMatches) -> Option<Box<dyn Executor>> {
    // handle subcommand ip;
    if let Some(matches) = matches.subcommand_matches("ip") {
        return IpExecutor::create(matches);
    }

    // default
    return DefaultExecutor::create(matches);
}


struct DefaultExecutor {
    
}
impl DefaultExecutor {
    // get args from command line
    fn create(matches: &ArgMatches) -> Option<Box<DefaultExecutor>> {
        Some(Box::new(DefaultExecutor{}))
    }
}    
impl Executor for DefaultExecutor {
    // execute command
    fn exe(&self) {
        println!("Welcome!");
    }   
}


struct IpExecutor {
    address: String,
}
impl IpExecutor {
        // get args from command line
    fn create(matches: &ArgMatches) -> Option<Box<IpExecutor>>{
        let address = matches.value_of("ADDRESS").unwrap();
        Some(Box::new(IpExecutor{address:address.to_string()}))
    }
    
    fn query_location(&self)  {
        // load source
        let ak = env::var("BAIDU_MAP_AK").expect("expect ak");
        let sk = env::var("BAIDU_MAP_SK").expect("expect sk");
        let ip = &self.address;
        
        // baidu api https://api.map.baidu.com/location/ip?ak=您的AK&ip=您的IP&coor=bd09ll        
        let whole_str = format!("/location/ip?ak={}&ip={}&coor={}{}", urlencoding::encode(&ak), urlencoding::encode(ip),
                urlencoding::encode("bd09ll"),sk);
        let temp_str = urlencoding::encode(&whole_str);
        let digest = md5::compute(temp_str);
        let sn = format!("{:x}", digest);

        // format url
        let url = format!("https://api.map.baidu.com/location/ip?ak={}&ip={}&coor=bd09ll&sn={}",
                          ak, ip, sn);
        let body = reqwest::blocking::get(&url).unwrap()
            .json::<serde_json::Value>().unwrap();
        println!("{}, {}",  body["content"]["address_detail"]["province"].as_str().unwrap()
                 , body["content"]["address_detail"]["city"].as_str().unwrap());
    }
}
impl Executor for IpExecutor {
    // execute command
    fn exe(&self) {
        self.query_location();            
    }
}
