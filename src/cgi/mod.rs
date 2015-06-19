/* cgi POST and GET
	Author: Al Poole <netstar@gmail.com>
*/

use std::io::{Read};
use std::io;
use std::env;
use std::string::{String};
use std::collections::HashMap;

const CHUNK: usize = 4192;

pub struct Cookie {
	pub name: String,
	pub value: String,
	pub expiry: u32,
	pub path: String,
	pub domain: String,
	is_new: bool,
}

impl Cookie {

pub fn new() -> Cookie
{
	return Cookie {
		name: "".to_string(),
		path: "".to_string(),
		domain: "".to_string(),
		value: "".to_string(),
		expiry: 0,
		is_new: true,
	}
}
}

pub struct Cgi {
	params: HashMap<String, String>,
	pub cookies: Vec<Cookie>,
}

impl Cgi {

fn get_pushed_data(data: Vec<u8>, length: usize) -> HashMap<String, String>
{
	let buf = data;
	let mut i = 0;
	let mut parameters = HashMap::new();

	while i < length 
	{
		let mut name: String = String::new();
		let mut data: String = String::new();
	
		while buf[i] as char != '='
		{
			name.push(buf[i] as char);			
			i += 1;
		}

		i += 1;
		
		while i < length && buf[i] as char != '&' 
		{
			data.push(buf[i] as char);
			i += 1;
		}
		i += 1;
		parameters.insert(name, data);
	}

	return parameters;
}


fn find_header(buf: &mut [u8;65535], needle: &str, byte: u8) -> String
{
	let mut i = 0;
	let mut x = 0;
	let mut request = String::new();
	let srch = needle.as_bytes();

	if srch.len() == 0 
	{
		return request;
	}	
	
	while buf[i] as char != '\0' 
	{
		if buf[i] as char == srch[0] as char 
		{
			while x < srch.len() && buf[i] as char  == srch[x] as char 
			{
				i = i + 1;
				x = x + 1;
			}
			break;
		} else {
			i = i + 1;
		}
	}

	if x == srch.len() 
	{	
		//println!("match");
		let mut end = i;
		while buf[end] as char != byte as char && buf[end] as char != '\n' 
		{
			end = end + 1;

		}

		for y in (i..end) 
		{	
			request.push(buf[y] as char);
		}
	
		return request;
	}	
	return request;
} 


fn get_http_cookies() -> Vec<Cookie> {
	let method = "";
	let mut cookies: Vec<Cookie> = Vec::new();

	match env::var("HTTP_COOKIES")
	{
		Ok(_) => 
		{

		}
		Err(_) =>
		{
			return cookies;
		}
	}


	let buf = method.as_bytes();
	
	
	let mut last_cookie = 0;

	let mut line = [0u8; 65535];

	let mut i = 0;

	while last_cookie != 1 {
		let mut name: String = String::new();
		let mut value: String = String::new();

		while buf[i] as char != '='
		{
			name.push(buf[i] as char);			
			i += 1;
		}
		
		i = i + 1;

		while buf[i] as char != ';' && buf[i] as char != '\0'
		{
			value.push(buf[i] as char);
			i += 1;
		}

		if buf[i] as char == '\0'
		{
			last_cookie = 1;
		}

		let mut len = 0;
	
		while buf[i] as char != '\n'
		{
			line[len] = buf[i];
			len += 1; i += 1;
		}
	
		let expires = Cgi::find_header(&mut line, "expires=", ';' as u8);
		let expiry: u32  = expires.trim().parse().unwrap();

		let path = Cgi::find_header(&mut line,  "path=", ';' as u8);
		let domain = Cgi::find_header(&mut line, "domain=", ';' as u8);
	
		cookies.push(Cookie{name: name, value: value, expiry: expiry, path: path, domain: domain, is_new: false});
	}

	return cookies;
}

pub fn cookie_get(&self, name: String)  -> Cookie {
	let mut cookie = Cookie::new();

	for c in self.cookies.iter()
	{
		if ! c.name.is_empty() && c.name == name.to_string()
		{
			cookie.name = c.name.to_string(); //  = c.clone();
			cookie.value = c.value.to_string();
			cookie.path  = c.path.to_string();
			cookie.domain = c.domain.to_string();
			cookie.expiry = c.expiry;
			return cookie;	
		}
	}

	return cookie; // empty 
}

pub fn cookies_set(&self, content: &'static str) {

        for c in self.cookies.iter()
        {
		if c.is_new && !c.name.is_empty() // BUG?
		{
			print!("Set-Cookie: ");
			let mut output = format!("{}={};", c.name, c.value);
			println!("{}", output);
			if c.expiry > 0
			{	
				output = format!("expires={};", c.expiry); 
				println!("{}", output);
			}
		
			if ! c.path.is_empty()
			{
				output = format!("path={};", c.path);
				println!("{}", output);
			}

			if ! c.domain.is_empty()
			{
				output = format!("domain={};", c.domain);
				println!("{}", output);
			}
       		 }
	
	}
	println!("Content-Type: {}\r\n\r\n", content);
}

fn get_http_request() -> HashMap<String,String> {
	let mut method = env::var("REQUEST_METHOD").unwrap();
	let mut contents: Vec<u8> = Vec::new();
	let mut length = 0;

	match method.as_ref()
	{
		"POST" =>
		{
			method = env::var("CONTENT_LENGTH").unwrap();
			let total: usize = method.trim().parse().unwrap();
			let mut buf = [0u8; CHUNK];

			let mut current = 0;

			let mut stream = io::stdin(); //File::open("/dev/stdin").unwrap();
			while current < total 
			{
				let bytes = stream.read(&mut buf[0..CHUNK]).unwrap();
				current += bytes;
				
				for byte in buf.iter() {
					contents.push(*byte);
					length += 1;
				}
			}
		}
		
		"GET" =>
		{
			let query_string = env::var("QUERY_STRING").unwrap();
			
			for byte in query_string.as_bytes() 
			{
				contents.push(*byte);
				length += 1;
			}	
			
		}


		_ =>
		{

		}
	}
	

	Cgi::get_pushed_data(contents, length)
}



pub fn param(&self, key: &'static str) -> String
{
	let mut result = String::new();

	for (name, value) in self.params.iter()
	{
		if name == key {
			result.push_str(value);
			return result;
		}
	}
	panic!("uh oh")
}

pub fn new() -> Cgi
{
	let params = Cgi::get_http_request();
	let cookies = Cgi::get_http_cookies();

	return Cgi {
		params: params,
		cookies: cookies,
	}
}

}

