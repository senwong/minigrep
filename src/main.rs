use std::{env, process, iter::Map, collections::HashMap, thread};

use w_grep::{Config, run};


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap();
    run(&config).unwrap_or_else(|e| eprintln!("执行失败"));
}
