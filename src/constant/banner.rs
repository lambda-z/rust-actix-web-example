
trait BannerTrait {
    fn print_banner(&self);
    fn new() -> Self;
}


struct Banner {
    pub banner: String
}

impl BannerTrait for Banner {
    fn print_banner(&self) {
        println!("{}", self.banner);
    }

    fn new() -> Self {
        Banner {
            banner: "".to_string()
        }
    }
}