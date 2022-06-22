mod lib;

use lib::search_dic;

fn main() {
    for i in search_dic("./", true) {
        println!("{}", i.display())
    }
}


