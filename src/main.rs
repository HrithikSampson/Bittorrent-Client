
//use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;
//use serde::ser::SerializeSeq;
use serde_json;
//use serde_json::json;
use std::env;
use std::fs;
use serde_bencode;
use serde_bencode::value::Value;
use serde::{Serializer,de::Visitor, Deserializer};
use sha1::{Digest, Sha1};
// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number
    let decode: serde_bencode::value::Value = serde_bencode::from_str::<Value>(&encoded_value).unwrap_or_else(|error|{
        panic!("Error decoding value to bencode:{}",error);
    });
    //println!("{:?}",decode);
    let decoded_value:serde_json::Value= match decode {
        serde_bencode::value::Value::Bytes(bytes) => {serde_json::value::Value::String(String::from_utf8_lossy(&bytes).into_owned())},
        serde_bencode::value::Value::List(list) => {list.into_iter()
            .map(|element|  decode_bencoded_value(&serde_bencode::to_string::<Value>(&element)
            .unwrap_or_else(|error|{
                panic!("Error decoding value to bencode:{}",error);
            }))).collect()},
        serde_bencode::value::Value::Dict(dict) => {serde_json::value::Value::Object(dict.into_iter()
            .map(|(key,value)| (String::from_utf8_lossy(&key).into_owned(),
            decode_bencoded_value(&serde_bencode::to_string(&value).unwrap())))
            .collect())},
        _ => serde_json::to_value(&decode).unwrap_or_else(|error|{
            panic!("Error converting bencode to value format:{}",error);
        }),
    };
    decoded_value
}
#[derive(Debug,PartialEq, Eq)]
struct Hashes(Vec<[u8; 20]>);
#[derive(Serialize,Deserialize,Debug,PartialEq, Eq)]
struct Info{
    length: usize,
    name: String,
    #[serde(rename="piece length")]
    piece_length: usize,
    pieces: Hashes,
}


#[derive(Serialize,Deserialize,Debug,PartialEq, Eq)]
struct TorrentFile{
    announce: String,
    info: Info

}
#[derive(Debug)]
struct HashesVisitor;
impl Serialize for Hashes
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
         
        serializer.serialize_bytes(&self.0.concat())
    }
}
impl<'de> Visitor<'de> for HashesVisitor {
    type Value = Hashes;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        //println!("{:?}",self);
        formatter.write_str("a byte string whose length is multiple of 20")
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // println!("{:?}",v);
        Ok(Hashes(
            v.chunks_exact(20)
            .map(|chunk| chunk.try_into().expect("guaranteed to be length 20"))
            .collect(),
        ))
         
    } 
}
impl<'de> Deserialize<'de> for Hashes {
    fn deserialize<D>(deserializer: D) -> Result<Hashes, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(HashesVisitor)
    }
}
// fn encode_hex(bytes: &[u8]) -> String {
//     bytes
//         .iter()
//         .map(|byte| format!("{:02x}", byte))
//         .collect()
// }
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    }
    else if command == "info" {
        let file_path = &args[2];
        let contents = fs::read(file_path)
        .expect("Should have been able to read the file");
        let content_torrent:TorrentFile = serde_bencode::from_bytes(&contents).unwrap();
        // println!("{:?}",content_torrent.info);
        println!("Tracker URL: {}", &content_torrent.announce.as_str());
        println!("Length: {}", &content_torrent.info.length);
        let bencoded_info = serde_bencode::to_bytes(&content_torrent.info).unwrap();//serde_bencode::to_string(&{..content_torrent.info}).unwrap();
        //println!("{:?}",bencoded_info);
        // let decoded_info = decode_bencoded_value(bencoded_info.as_str());
        //let decoded_info = decode_bencoded_value(bencoded_info.as_str());
        //println!("{:?}",content_torrent.info);
        //println!("{:?}",decoded_info);
        let mut hasher = Sha1::new();
        // let decode = match decoded_info{
        //     serde_json::value::Value::Object(s)=>json!(s),
        //     _ => json!([])
        // };

        //let decode = serde_json::to_string(&decoded_info).unwrap();
        //println!("{:?}",decode);
        
        hasher.update(&bencoded_info);
        let hash = hasher.finalize(); 
        println!("Info Hash: {}",hex::encode(&hash));
    } else {
        println!("unknown command: {}", args[1])
    }
}
