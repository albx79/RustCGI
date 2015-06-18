extern crate WebFoolKit;

fn main() {
	println!("Content-Type: text/html\r\n\r\n");
 	println!("<h1>hi</h1>");

	let c = WebFoolKit::Cgi::new();

	let value = c.param("data");

	println!("<p>value is {} </p>", value);	

	println!("<p>DONE!</p>");
}
