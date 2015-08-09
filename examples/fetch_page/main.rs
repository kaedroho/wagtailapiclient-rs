extern crate wagtailapiclient;

use wagtailapiclient::client::WagtailClient;
use wagtailapiclient::query::WagtailQuery;


fn main() {
    let client = WagtailClient::new("http://wagtailapi.kaed.uk/api/v1/");

    // Get homepage (usually first page)
    let homepage = client.pages().limit(1).iter().nth(0).unwrap();

    // Get children of homepage
    let homepage_children = homepage.get_children();

    for page in homepage_children.iter() {
        println!("{}", page.title);
    }
}
