
use serde_json;
use std::env;
use serde_bencode;
use serde_bencode::value::Value;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number

    let decode: serde_bencode::value::Value = serde_bencode::from_str::<Value>(&encoded_value).unwrap_or_else(|error|{
        panic!("Error decoding value to bencode:{}",error);
    });
    let decoded_value:serde_json::Value= match decode {
        serde_bencode::value::Value::Bytes(bytes) => {serde_json::value::Value::String(String::from_utf8_lossy(&bytes).into_owned())},
        serde_bencode::value::Value::List(list) => {list.into_iter()
            .map(|element|  decode_bencoded_value(&serde_bencode::to_string::<Value>(&element)
            .unwrap_or_else(|error|{
                panic!("Error decoding value to bencode:{}",error);
            }))).collect()}
        _ => serde_json::to_value(&decode).unwrap_or_else(|error|{
            panic!("Error converting bencode to value format:{}",error);
        }),
    };
    decoded_value
}

// fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
//     // If encoded_value starts with a digit, it's a number
//     if encoded_value.chars().next().unwrap().is_digit(10) {
//         // Example: "5:hello" -> "hello"
//         let colon_index = encoded_value.find(':').unwrap();
//         let number_string = &encoded_value[..colon_index];
//         let number = number_string.parse::<i64>().unwrap();
//         let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
//         return serde_json::Value::String(string.to_string());
//     } else {
//         panic!("Unhandled encoded value: {}", encoded_value)
//     }
// }
// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        //println!("Logs from your program will appear here!");

        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
