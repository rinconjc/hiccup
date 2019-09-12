#[derive(Debug)]
struct Tag{
    name:String,
    // props:HashMap<String,Any>,
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
    // ($tag:ident) => {Tag{name: stringify!($tag).into(), children:vec!()}};
    (@empty ()) => {};
    (@props $($k:ident : $v:expr),*) => {
        {
            let mut m = HashMap::new();
            $(m.insert(stringify!($k), $v);)*
                m
        }
    };
    ($tag:ident) => {hiccup!($tag {})};

    ($tag:ident {$props:pat}) => {hiccup!($tag {$props})};

    ($tag:ident $([$children:pat])+) => {hiccup!($tag {} $([$children])+)};

    ($tag:ident {$($props:tt)*} $([$($child:tt)*])*) => {
        {
            let props = hiccup!(@props $($props)*);
              println!("parsed props {:?}", props);

            let mut v = Vec::new();
            $(v.push(hiccup!($($child)*));)*
                Tag{name: stringify!($tag).into(), children:v}
        }
    };
}

fn main() {
    println!("Hello, world! {:?}", Tag{name:"div".into(), children:vec![Tag{name:"span".into(), children:vec![]}]});
    let t = hiccup![br];
    let t2 = hiccup![div { prop : "a"} [span {x:"1"}] [a]];
    println!("Tag: {:?}", t2);
    let m = kvmap!{"1" : 2, "3" : 4, "abra cadabra": 5};
    println!("map1: {:?}, other, {:?}", m,  kvmap!{1:2, 3:4+5});
    println!("Map:{:?}", kvmap!{a:2, b:4});
}
