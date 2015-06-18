/* cgi POST and GET
	Author: Al Poole <netstar@gmail.com>
*/

use std::io::{Read};
use std::io;
use std::env;
use std::string::{String};
use std::collections::HashMap;

const CHUNK: usize = 4192;

pub struct Cgi {
	params: HashMap<String, String>,
	cookies: HashMap<String, String>,
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

fn get_http_cookies() -> HashMap<String,String> {
	let mut contents: Vec<u8> = Vec::new();
	let mut cookies = HashMap::new();

	return cookies;
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
	return Cgi {
		cookies: Cgi::get_http_cookies(),
		params: Cgi::get_http_request(),
	}
}

}

