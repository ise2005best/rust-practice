// hides warnings
#![allow(dead_code)]

#[derive(Debug)]


struct Person{
    name: String,
    age: u8
}

struct Unit;

struct Pair (i32, f32);

struct Point{
    x: f32,
    y : f64
}

struct Rectangle{
    top_left: Point,
    top_right: Point
}

pub fn main(){
    let name: String = String::from("Isee");
    let age: u8 = 18;
    let my_name = Person{name,age};
    println!("{:?}", my_name);

    let point: Point = Point { x: (3.5), y: (10.2) };
    println!("point coordinates {} {}", point.x, point.y);
    
    // this creates a new point and let the value of x be 5.2 and the value of y be the previous value
    let borrom_right = Point{x:5.2, ..point};
    println!("{}", borrom_right.y );

    // this is how destructuring works in rust
    let Point {x: left_edge, y: top_edge} = borrom_right;
    print!("{} {}", left_edge, top_edge);

    let _rectangle = Rectangle{
        top_left: Point { x: (left_edge), y: (top_edge) },
        top_right: borrom_right
    };

    println!("the coordinates are {} and {}", _rectangle.top_left.x, _rectangle.top_left.y);
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
    Click {x: i64, y:i64},
}

fn inspect(event: WebEvent){
    // match like an if statemenet
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted \"{} \" .", s),
        WebEvent::Click { x, y } =>{
            println!("clicked at x= {}, y = {}", x,y);
        },

    }
}

pub fn enums() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}