use std::cmp;
use std::rc::Rc;
use std::clone::Clone;

use rustc_serialize::json;

use iterator::WagtailIterator;
use client::WagtailClient;


pub trait WagtailQuery: Clone {
    type Item;

    fn get_client(&self) -> Rc<WagtailClient>;
    fn get_start_stop(&self) -> (usize, Option<usize>);
    fn set_start_stop(&mut self, start: usize, stop: Option<usize>);
    fn get_endpoint_url(&self) -> String;
    fn results_attr_name(&self) -> String;
    fn process_item(&self, item: &json::Object) -> Self::Item;

    fn offset(&self, n: usize) -> Self {
        let mut clone = self.clone();

        let (mut start, stop) = clone.get_start_stop();
        start += n;
        clone.set_start_stop(start, stop);

        clone
    }

    fn limit(&self, n: usize) -> Self {
        let mut clone = self.clone();

        let (start, mut stop) = clone.get_start_stop();
        stop = Some(
            match stop {
                Some(old_stop) => {
                    start + cmp::min(n, old_stop - start)
                },
                None => start + n
            }
        );
        clone.set_start_stop(start, stop);

        clone
    }

    fn fetch(&self) -> Option<Vec<Self::Item>> {
        let json = self.get_client().get(&self.get_endpoint_url());
        let mut items = Vec::new();

        let obj = json.as_object().unwrap();
        let results = obj.get(&self.results_attr_name()).unwrap().as_array().unwrap();

        for result in results {
            let result_obj = result.as_object().unwrap();

            items.push(
                self.process_item(result_obj)
            );
        }

        Some(items)
    }

    fn iter(&self) -> WagtailIterator<Self> {
        WagtailIterator::new(self.clone())
    }
}
