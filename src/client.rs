use std::rc::Rc;

use endpoints::pages::PageQuery;


#[derive(Clone)]
pub struct WagtailClient {
    base_url: String
}


impl WagtailClient {
    pub fn new(base_url: &str) -> WagtailClient {
        WagtailClient{
            base_url: base_url.to_owned()
        }
    }

    pub fn pages(&self) -> PageQuery {
        PageQuery::new(Rc::new(self.clone()))
    }
}
