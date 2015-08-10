use std::rc::Rc;
use std::io::Read;

use hyper::client::Client;
use rustc_serialize::json;

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

    pub fn get(&self, url: &str) -> json::Json {
        let full_url = self.base_url.clone() + url;

        let client = Client::new();
        let mut response = client.get(&full_url).send().unwrap();

        let mut data = String::new();
        response.read_to_string(&mut data).unwrap();

        json::Json::from_str(&data).unwrap()
    }
}
