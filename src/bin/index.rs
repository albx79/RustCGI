extern crate WebFoolKit;

fn main() {
	let mut c = WebFoolKit::Cgi::new();
	let name = c.param("name");
	let value = c.param("value");

	let mut cookie = WebFoolKit::Cookie::new();
	cookie.name = name.to_string();
	cookie.value = value.to_string();

	c.cookies.push(cookie);
	c.cookies_set("text/html");

	let get_cookie = c.cookie_get(name);

 	println!("<h1>hi {}</h1>", get_cookie.name);


	println!("<p>value is {} </p>", value);	

	println!("<p>DONE!</p>");
}
