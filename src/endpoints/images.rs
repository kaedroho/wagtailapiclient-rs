use std::rc::Rc;

use rustc_serialize::json;

use query::WagtailQuery;
use client::WagtailClient;


#[derive(Clone)]
pub struct ImageMeta {
    pub content_type: String,
    pub detail_url: String,
}


#[derive(Clone)]
pub struct Image {
    client: Rc<WagtailClient>,
    pub id: usize,
    pub title: String,
    pub meta: ImageMeta,
}


#[derive(Clone)]
pub struct ImageQuery {
    client: Rc<WagtailClient>,
    start: usize,
    stop: Option<usize>,
}


impl ImageQuery {
    pub fn new(client: Rc<WagtailClient>) -> ImageQuery {
        ImageQuery {
            client: client.clone(),
            start: 0,
            stop: None,
        }
    }
}


impl WagtailQuery for ImageQuery {
    type Item = Image;

    fn get_start_stop(&self) -> (usize, Option<usize>) {
        (self.start, self.stop)
    }

    fn set_start_stop(&mut self, start: usize, stop: Option<usize>) {
        self.start = start;
        self.stop = stop;
    }

    fn get_endpoint_url(&self) -> String {
        "http://wagtailapi.kaed.uk/api/v1/images/".to_owned()
    }

    fn results_attr_name(&self) -> String {
        "images".to_owned()
    }

    fn process_item(&self, item: &json::Object) -> Self::Item {
        let meta = item.get("meta").unwrap().as_object().unwrap();

        Image{
            client: self.client.clone(),
            id: item.get("id").unwrap().as_u64().unwrap() as usize,
            title: item.get("title").unwrap().as_string().unwrap().to_owned(),
            meta: ImageMeta{
                content_type: meta.get("type").unwrap().as_string().unwrap().to_owned(),
                detail_url: meta.get("detail_url").unwrap().as_string().unwrap().to_owned(),
            }
        }
    }
}
