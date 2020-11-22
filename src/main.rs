// (my app for myself us as daily works)
extern crate clap;
extern crate dotenv;
extern crate urlencoding;

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


    // handle subcommand ip;
    if let Some(matches) = matches.subcommand_matches("ip") {
        let mut exe = IpExecutor{
            address: "".to_string(),
        };
        exe.analyseMatch(&matches);
        exe.exe();
    }
}


trait Executor {
    fn analyseMatch(&mut self, args: &ArgMatches);
    fn exe(&self);        
}


struct IpExecutor {
    address: String,
}

impl IpExecutor {
    fn queryLocation(&self)  {        
        let ak = env::var("BAIDU_MAP_AK").expect("DATABASE_URL must be set");
        let sk = env::var("BAIDU_MAP_SK").expect("sk")
        println!("ak{}", ak);
     
        // baidu api https://api.map.baidu.com/location/ip?ak=您的AK&ip=您的IP&coor=bd09ll

        let wholeStr = format!("/location/ip?ak={}&ip={}&coor={}{}", urlencoding::encode(ak), urlencoding::encode("11.238.18.36"),
                urlencoding::encode("bd09ll"),sk);

        let tempStr = urlencoding::encode(wholeStr);
        let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
        let sn = format!("{:x}", digest);

        let url = format!("https://api.map.baidu.com/location/ip?ak={}&ip={}&coor=bd09ll&sn={}",
                          ak, ip, sn);

        // todo request
    }
}

impl Executor for IpExecutor {

    // get args from command line
    fn analyseMatch(&mut self, matches: &ArgMatches) {
        let address = matches.value_of("ADDRESS").unwrap();
        self.address = address.to_string();
    }

    // execute command
    fn exe(&self) {
        println!("ip is {}", self.address);
        self.queryLocation();
            
    }
}
