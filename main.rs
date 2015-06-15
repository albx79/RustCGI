use std::io::{Read};
use std::io;
use std::env;
use std::string::{String};
use std::fs::File;
use std::vec;
use std::collections::HashMap;

const CHUNK: usize = 4192;

fn get_pushed_data(data: Vec<u8>) -> HashMap<String, String>
{
	let mut buf = data;
	let mut i = 0;
	let mut parameters = HashMap::new();

	while i < buf.len() 
	{
		let mut name: String = String::new();
		let mut data: String = String::new();
	
		while buf[i] as char != '='
		{
			name.push(buf[i] as char);			
			i += 1;
		}

		while buf[i] as char != '&' 
		{
			data.push(buf[i] as char);
			i += 1;
		}
		
		parameters.insert(name, data);
	}

	return parameters;
}

fn get_http_request() {
	let mut method = env::var("REQUEST_METHOD").unwrap();
	let mut contents: Vec<u8> = Vec::new();

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
				}
			}

		}
		
		"GET" =>
		{
			let query_string = env::var("QUERY_STRING").unwrap();
			
			for byte in query_string.as_bytes() 
			{
				contents.push(*byte);
			}	
			
		}


		_ =>
		{


		}
	}
	
	let mut params = get_pushed_data(contents);
}


fn main() {
    get_http_request();

    println!("Content-Type: text/html\r\n\r\n");
    println!("<h1>hi</h1>");

}
