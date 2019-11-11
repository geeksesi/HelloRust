// we need check url is http or https ?
// it's easy
fn check_is_url(url: String) -> bool {
    if (url.find("http://") != None) || (url.find("https://") != None) {
        return true;
    } else {
        return false;
    }
}

fn main() {
    // some test case
    println!("{:#?}", check_is_url("https://google.com/".to_string()));
    println!("{:#?}", check_is_url("localhost".to_string()));
    println!("{:#?}", check_is_url("google.com".to_string()));
    println!("{:#?}", check_is_url("http:github.com".to_string()));
    println!("{:#?}", check_is_url("http://github.com".to_string()));
}
