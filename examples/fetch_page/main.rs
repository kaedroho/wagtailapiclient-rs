extern crate wagtailapi;

use wagtailapi::query::WagtailQuery;
use wagtailapi::pages::PageQuery;


fn main() {
    let pq = PageQuery::new();

    // Get first page
    let page = pq.limit(1).iter().nth(0);
    println!("{:?}", page);
}
