use std::iter::Iterator;

use query::WagtailQuery;


pub struct WagtailIterator<Q: WagtailQuery> {
    query: Box<Q>,
    item_buffer: Option<Vec<Q::Item>>,
}


impl <Q: WagtailQuery> WagtailIterator<Q> {
    pub fn new(query: &Q) -> WagtailIterator<Q> {
        WagtailIterator{
            query: Box::new(query.clone()),
            item_buffer: None,
        }
    }
}


impl <Q: WagtailQuery> Iterator for WagtailIterator<Q> {
    type Item = Q::Item;

    fn next(&mut self) -> Option<Q::Item> {
        match self.item_buffer {
            None => {
                // Item buffer not yet populated, fetch it from server
                match self.query.fetch() {
                    Some(mut items) => {
                        items.reverse();
                        self.item_buffer = Some(items);
                        self.next()
                    }
                    None => None
                }
            },
            Some(ref mut item_buffer) => {
                // Return next item from buffer
                item_buffer.pop()
            }
        }
    }
}
