extern crate WebFoolKit;

fn main() {
	let c = WebFoolKit::Cgi::new();
	let value = c.param("data");
	let mut cookie = WebFoolKit::Cookie::new();

	c.cookies.push(cookie);

	c.cookies_set("text/html".to_string());

 	println!("<h1>hi</h1>");


	println!("<p>value is {} </p>", value);	

	println!("<p>DONE!</p>");
}
