/*
	Start of basic CGI for Rust.
	Author: Al Poole <netstar@gmail.com>

	TODO: Everything!!!

*/

use std::io::{Read};
use std::io;
use std::env;
use std::string::{String};
use std::fs::File;
use std::vec;
use std::collections::HashMap;

const CHUNK: usize = 4192;


struct CGI {
        params: HashMap<String,String>,
}

impl CGI {

pub fn new(&self) -> CGI
{
       self.params = self.get_http_request(); 
}

fn get_pushed_data(self, data: Vec<u8>, length: usize) -> HashMap<String, String>
{
	let mut buf = data;
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

fn get_http_request(self) -> HashMap<String,String> {
	let mut method = env::var("REQUEST_METHOD").unwrap();
	let mut contents: Vec<u8> = Vec::new();
	let mut length = 0;

	match method.as_ref()
	{
		"POST" =>
		{
			method = env::var("CONTENT_LENGTH").unwrap();
			let mut total: usize = method.trim().parse().unwrap();
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
			
			println!("<p>");
			for i in contents.iter() 
			{
				print!("{}", *i as char);
			}
			println!("</p");	
			println!("<p>POST</p>");

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
	

	self.get_pushed_data(contents, length)
}



pub fn param(&self, key: String) -> String
{
	let result = String::new();

		
	for (name, value) in self.params.iter()
	{
		if name == key
		{
			result.push_str(value);
			return result;
	.	}
	
	}
}

}

fn main() {
	println!("Content-Type: text/html\r\n\r\n");
 	println!("<h1>hi</h1>");

	let mut cgi = CGI::new();

	let value = cgi.param("name".to_string());

	println!("<p>value is {} </p>", value);	

	println!("<p>DONE!</p>");
}
