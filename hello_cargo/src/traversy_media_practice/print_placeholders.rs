pub fn run(){

    // positional arguments this prints out the respective arguments
    println!("{0} this is me trying to learn rust, i am also {1}, i also {2} {1}", "Dr Pius", "starting to like rust", "love");

    // Named arguments
    println!("Hello my name is {name} and i am in {class}", name = "Iseoluwa", class = "300 level");

    // the placeholders to print in binary, octal and hexadecimal
    println!("In binary = {:b} In Octal = {:o} In Hexadecimal = {:x}", 10,10,10);

    

}

pub fn assignment() {
    const PI: f32 = 3.14159;
    // a trait for circle that has the variable radius
    trait CircleTraits {
        fn new_circle(radius: f32) -> Self;
        // borrows self and passes it as a parameter 
        fn area_of_circle(&self) -> f32;
        fn perimeter_of_circle(&self) -> f32;
    }
    // i defined a trait for triangle that has the variables side1, side2, base and height
    trait TriangleTraits {
        fn new_triangle(side1: f32, base: f32, side2: f32, height: f32) -> Self;
        // borrows self and passes it as a parameter
        fn area_of_triangle(&self) -> f32;
        fn perimeter_of_triangle(&self) -> f32;
    }
    // the default macro is used here so we can set a default value for each variable of the struct circle while the debug is used to print out the entire struct
    #[derive(Default,Debug)]
    // struct for circle
    struct Circle {
        radius: f32,
    }
       // the default macro is used here so we can set a default value for each variable of the struct triangle while the debug is used to print out the entire struct
    #[derive(Default,Debug)]
    struct Triangle {
        side1: f32,
        side2: f32,
        base: f32,
        height: f32,
    }

    impl Circle {
        // we set the default value of circle
        fn default() -> Self {
            Circle { radius: 6.0 }
        }
    }

    impl Triangle {
        // we set the default value of triangle
        fn default() -> Self {
            Triangle {
                side1: 4.0,
                side2: 9.4,
                base: 7.0,
                height: 12.0,
            }
        }
    }

    impl CircleTraits for Circle {
        // we create a new circle that we would use for comparisons
        fn new_circle(radius: f32) -> Self {
            Circle { radius }
        }
        // a function for the area of circle which is pie r square which we defined here
        fn area_of_circle(&self) -> f32 {
            PI * self.radius * self.radius
        }
        // a function for the perimeter of a circle which is 2 * pie * r
        fn perimeter_of_circle(&self) -> f32 {
            2.0 * PI * self.radius
        }
    }

    impl TriangleTraits for Triangle{
        // a function to create a new triangle that would be used for comparison
        fn new_triangle(side1: f32, base: f32, side2: f32, height: f32) -> Self {
            Triangle { side1, 
                side2, 
                base, 
                height, 
            }
        }
        // a function to create the area of a triangle
        fn area_of_triangle(&self) -> f32 {
            (self.base * self.height) / 2.0
        }
        // a function to create the perimeter of a triangle
        fn perimeter_of_triangle(&self) -> f32 {
            self.side1 + self.side2 + self.base
        }
    }

    // a trait used for comparison that has two methods eq and ne
    impl PartialEq for Circle{
        // the method that compares if the two radiuses are equals to one another
        fn eq(&self, other: &Self) -> bool {
            self.radius == other.radius
        }
        // the method that compares if the two radiuses are equals to one another
        fn ne(&self, other: &Self) -> bool {
            !self.radius.eq(&other.radius)
        }
    }
    impl PartialOrd for Circle{
        // this returns an ordering that returns either less, greater or equals 
        // it does a comparison between each radius
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.radius.partial_cmp(&other.radius)
        }
        // comparison between each area of circles and check if it is greater than or equals to
        fn ge(&self, other: &Self) -> bool {
            self.perimeter_of_circle() == other.perimeter_of_circle()
        }
        // i would be stopping here for all the methods of partialOrd the other methods are being implemented with triangle
    }
    impl PartialEq for Triangle{
        // check if the two bases are equal, returns a boolean value, true if they are equal and false if they arent equal
        fn eq(&self, other: &Self) -> bool {
            self.base == other.base
        }
        // check if the two triangles bases are not equal 
        fn ne(&self, other: &Self) -> bool {
            !self.base.eq(&other.base)
        }
    }
    impl PartialOrd for Triangle{
        
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.perimeter_of_triangle().partial_cmp(&other.perimeter_of_triangle())
        }
       
    }

    let circle1 = Circle::default();
    let circle2 = Circle::new_circle(7.5);
    println!("The values of circle1 are {:?}", circle1);
    println!("The area of the circle is {}", circle1.area_of_circle());
    println!("The perimeter of the circle is {}", circle1.perimeter_of_circle());

   
    let partial_cmp_comparison = circle1.partial_cmp(&circle2);
    println!("The comparison between the two circles radius's is {:?} ", partial_cmp_comparison);
    println!("Do circle1 and circle2 have the same radius {}", circle1.eq(&circle2));
    println!("Do circle1 and circle2 have different radius {}", circle1.ne(&circle2));
    println!("Is circle1 perimeter greater than or equals to circle2 perimeter {}", circle1.perimeter_of_circle().ge(&circle2.perimeter_of_circle()));


    let triangle1 = Triangle::default();
    let triangle2 = Triangle:: new_triangle(4.9, 8.5, 21.0, 5.0);

    println!("The values of Triangle1 are {:?}",triangle1 );
    println!("The area of the circle is {}",triangle1.area_of_triangle());
    println!("The perimeter of the circle is {}", triangle1.perimeter_of_triangle());
    let partial_cmp_comparison_triangle = triangle1.partial_cmp(&triangle2);
    println!("The comparison between the two triangles perimeter {:?} ", partial_cmp_comparison_triangle);
    println!("Do triangle1 and triangle2 have the same bases {}", triangle1.eq(&triangle2));
    println!("Do triangle1 and triangle2 have different bases {}", triangle1.ne(&triangle2));

}
