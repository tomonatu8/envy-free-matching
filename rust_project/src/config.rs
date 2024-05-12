

pub struct Config {
    pub n_each: usize,
    pub num_groups: usize,
    pub num_items: usize,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let n_each = match args[1].parse() {
            Ok(num) => num,
            Err(_) => return Err("invalid value for n_each"),
        };

        let num_groups = match args[2].parse() {
            Ok(num) => num,
            Err(_) => return Err("invalid value for n_each"),
        };

        let num_items = match args[3].parse() {
            Ok(num) => num,
            Err(_) => return Err("invalid value for num_items"),
        };

        Ok(Config {
            n_each,
            num_groups,
            num_items,
        })
    }
}