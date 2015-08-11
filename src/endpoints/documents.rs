use std::rc::Rc;

use rustc_serialize::json;

use query::WagtailQuery;
use client::WagtailClient;


#[derive(Clone)]
pub struct DocumentMeta {
    pub content_type: String,
    pub detail_url: String,
    pub download_url: String,
}


#[derive(Clone)]
pub struct Document {
    client: Rc<WagtailClient>,
    pub id: usize,
    pub title: String,
    pub meta: DocumentMeta,
}


#[derive(Clone)]
pub struct DocumentQuery {
    client: Rc<WagtailClient>,
    start: usize,
    stop: Option<usize>,
}


impl DocumentQuery {
    pub fn new(client: Rc<WagtailClient>) -> DocumentQuery {
        DocumentQuery {
            client: client,
            start: 0,
            stop: None,
        }
    }
}


impl WagtailQuery for DocumentQuery {
    type Item = Document;

    fn get_client(&self) -> Rc<WagtailClient> {
        self.client.clone()
    }

    fn get_start_stop(&self) -> (usize, Option<usize>) {
        (self.start, self.stop)
    }

    fn set_start_stop(&mut self, start: usize, stop: Option<usize>) {
        self.start = start;
        self.stop = stop;
    }

    fn get_endpoint_url(&self) -> String {
        "documents/".to_owned()
    }

    fn results_attr_name(&self) -> String {
        "documents".to_owned()
    }

    fn process_item(&self, item: &json::Object) -> Self::Item {
        let meta = item.get("meta").unwrap().as_object().unwrap();

        Document{
            client: self.client.clone(),
            id: item.get("id").unwrap().as_u64().unwrap() as usize,
            title: item.get("title").unwrap().as_string().unwrap().to_owned(),
            meta: DocumentMeta{
                content_type: meta.get("type").unwrap().as_string().unwrap().to_owned(),
                detail_url: meta.get("detail_url").unwrap().as_string().unwrap().to_owned(),
                download_url: meta.get("download_url").unwrap().as_string().unwrap().to_owned(),
            }
        }
    }
}


impl WagtailClient {
    pub fn documents(&self) -> DocumentQuery {
        DocumentQuery::new(Rc::new(self.clone()))
    }
}
