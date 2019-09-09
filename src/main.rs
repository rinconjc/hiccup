#[derive(Debug)]
struct Tag{
    name:String,
    children: Vec<Tag>,
}
use std::collections::HashMap;

macro_rules! kvmap{
    ($($k:ident : $v:expr),+) => {
        {
            let mut m = HashMap::new();
            $(m.insert(stringify!($k), $v);)+
                m
        }
    };
    ($($k:tt : $v:expr),+) => {
        {
            let mut m = HashMap::new();
            $(m.insert($k, $v);)+
                m
        }
    }
}

macro_rules! hiccup{
    ($tag:ident) => {Tag{name: stringify!($tag).into(), children:vec!()}};
    ($tag:ident $( { $($k:ident : $v:expr),* } )? $([$child:tt])*) => {
        {
            let mut v = Vec::new();
            $(v.push(hiccup!($child));)*
            Tag{name: stringify!($tag).into(), children:v}
        }
    };
}

fn main() {
    println!("Hello, world! {:?}", Tag{name:"div".into(), children:vec![Tag{name:"span".into(), children:vec![]}]});
    let t = hiccup![br];
    let t2 = hiccup![div { prop: "a"} [span { x : "1" }] [a]];
    println!("Tag: {:?}", t2);
    let m = kvmap!{"1" : 2, "3" : 4, "abra cadabra": 5};
    println!("map1: {:?}, other, {:?}", m,  kvmap!{1:2, 3:4+5});
    println!("Map:{:?}", kvmap!{a:2, b:4});
}
