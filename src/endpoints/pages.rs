use std::rc::Rc;

use rustc_serialize::json;

use query::WagtailQuery;
use client::WagtailClient;


#[derive(Clone)]
pub struct PageMeta {
    pub content_type: String,
    pub detail_url: String,
}


#[derive(Clone)]
pub struct Page {
    client: Rc<WagtailClient>,
    pub id: usize,
    pub title: String,
    pub meta: PageMeta,
}


impl Page {
    pub fn get_children(&self) -> PageQuery {
        PageQuery::new(self.client.clone()).child_of(&self)
    }
}


#[derive(Clone)]
pub struct PageQuery {
    client: Rc<WagtailClient>,
    start: usize,
    stop: Option<usize>,
    child_of_filter: Option<usize>,
}


impl PageQuery {
    pub fn new(client: Rc<WagtailClient>) -> PageQuery {
        PageQuery {
            client: client,
            start: 0,
            stop: None,
            child_of_filter: None
        }
    }

    pub fn child_of(&self, parent: &Page) -> PageQuery {
        let mut clone = self.clone();
        clone.child_of_filter = Some(parent.id);
        clone
    }
}


impl WagtailQuery for PageQuery {
    type Item = Page;

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
        "pages/".to_owned()
    }

    fn results_attr_name(&self) -> String {
        "pages".to_owned()
    }

    fn process_item(&self, item: &json::Object) -> Self::Item {
        let meta = item.get("meta").unwrap().as_object().unwrap();

        Page{
            client: self.client.clone(),
            id: item.get("id").unwrap().as_u64().unwrap() as usize,
            title: item.get("title").unwrap().as_string().unwrap().to_owned(),
            meta: PageMeta{
                content_type: meta.get("type").unwrap().as_string().unwrap().to_owned(),
                detail_url: meta.get("detail_url").unwrap().as_string().unwrap().to_owned(),
            }
        }
    }
}


impl WagtailClient {
    pub fn pages(&self) -> PageQuery {
        PageQuery::new(Rc::new(self.clone()))
    }
}
