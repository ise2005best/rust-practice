// hides warnings
#![allow(dead_code)]

#[derive(Debug)]

struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f64,
}

struct Rectangle {
    top_left: Point,
    top_right: Point,
}

pub fn main() {
    let name: String = String::from("Isee");
    let age: u8 = 18;
    let my_name = Person { name, age };
    println!("{:?}", my_name);

    let point: Point = Point {
        x: (3.5),
        y: (10.2),
    };
    println!("point coordinates {} {}", point.x, point.y);

    // this creates a new point and let the value of x be 5.2 and the value of y be the previous value
    let borrom_right = Point { x: 5.2, ..point };
    println!("{}", borrom_right.y);

    // this is how destructuring works in rust
    let Point {
        x: left_edge,
        y: top_edge,
    } = borrom_right;
    print!("{} {}", left_edge, top_edge);

    let _rectangle = Rectangle {
        top_left: Point {
            x: (left_edge),
            y: (top_edge),
        },
        top_right: borrom_right,
    };

    println!(
        "the coordinates are {} and {}",
        _rectangle.top_left.x, _rectangle.top_left.y
    );
    println!("{}", _rectangle.top_right.y);

    let pair: Pair = Pair(3, 4.0);
    let _unit = Unit;

    println!("The pair contains {} and {}", pair.0, pair.1);
}

// enums

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    // match like an if statemenet
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted \"{} \" .", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x= {}, y = {}", x, y);
        }
    }
}

pub fn enums() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

// we are defining a trait called shape that can be called in any struct
// we defined name as static so as to ensure the value is availanle throughout the lifetime of the program
trait Shape {
    fn new(length: i32, width: i32, name: &'static str) -> Self;
    fn area(&self) -> i32;
    fn set_length(&mut self, length: i32);
    fn get_length(&self) -> i32;
    fn set_width(&mut self, width: i32);
    fn get_width(&self) -> i32;
    fn set_name(&mut self, name: &'static str);
    fn get_name(&self) -> &str;
}

#[derive(Default, Debug, Clone)]

struct Rect {
    length: i32,
    width: i32,
    name: &'static str,
}

impl Rect {
    // set the default value of the rect
    fn default() -> Self {
        Rect {
            length: 32,
            width: 55,
            name: "Iseoluwa",
        }
    }
}

// implementing the shape trait on the struct rect
impl Shape for Rect {
    fn new(length: i32, width: i32, name: &'static str) -> Self {
        Rect {
            length,
            width,
            name,
        }
    }

    fn area(&self) -> i32 {
        self.length * self.width
    }

    fn set_length(&mut self, length: i32) {
        self.length = length;
    }

    fn get_length(&self) -> i32 {
        self.length
    }

    fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    fn get_width(&self) -> i32 {
        self.width
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

// partial eq used for comparison and has two methods eq and ne, eq is == while ne !=
impl PartialEq for Rect{
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// partial ord used for comparison and has 5 methods used for >=, <=, >, < 
impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

// this allows me to convert 
impl From <&'static str> for Rect{
    fn from(s: &'static str) -> Rect {
        // this expects the string to be seperated with commas so its splits them into parts anywhere a comma is expected
        let mut parts = s.split(',');
        let length = match parts.next(){
            // this parses the string and changes it to an integer of i32
            // if there is no next part set the length to 0
            Some(val) => val.parse:: <i32>().unwrap(),
            None => 0,
        };
        let width = match  parts.next() {
            Some(val) => val.parse:: <i32>().unwrap(),
            None => 0,
        };
        let name = match parts.next(){
            Some(val) => val,
            None => "",
        };
        Rect { length, width, name: &name }
    }
}

pub fn run_code() {
    // prints the default values of rect
    let rectangle1 = Rect::default();

    println!("{}", rectangle1.length);
    println!("{}", rectangle1.width);
    println!("{}", rectangle1.name);

    let rectangle2 = Rect::new(1, 3, "Rectangle2");
    let rectangle3 = Rect::from("90,43,iseoluwa");
    println!("rectangle3 = {:?}", rectangle3);
    
    let result1 = rectangle1.partial_cmp(&rectangle3);
    println!("result1 = {:?}", result1);
    

    // le is <= 
    let result2 = rectangle1.le(&rectangle2);
    println!("result2 = {:?}", result2);

     //Compare using PartialEq

     // eq is == 
     let result3 = rectangle2.eq(&rectangle3);
     println!("result3 = {:?}", result3);
 
    // ne is != 
     let result4 = rectangle2.ne(&rectangle3);
     println!("result4 = {:?}", result4);
}