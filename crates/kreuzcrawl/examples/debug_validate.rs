fn main() {
    let mut c = kreuzcrawl::CrawlConfig::default();
    c.max_depth = Some(200);
    println!("max_depth = {:?}", c.max_depth);
    match c.validate() {
        Ok(()) => println!("validate: OK"),
        Err(e) => println!("validate: ERR: {e}"),
    }
}
