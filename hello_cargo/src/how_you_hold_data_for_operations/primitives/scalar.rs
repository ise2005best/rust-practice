pub fn scalar_literals (){
    let logical :bool = true;

    let a_float: f64 = 1.0; // a float, regular annotation

    let an_integer = 5i32; // an integer, suffic annotation

    let default_float = 2.9; // a float with a default annotation with dataType f64

    let default_integer = 9; // an integer with a default annotation of dataType i32

    let mut inferred_type = 12;
    inferred_type = 234949303;

    let mut mutable = 12;
    mutable = 33;

    a_float;
    // mutable = true;
    // println!("{} {} {} {}", {mutable}, {inferred_type}, {an_integer}, {default_integer});

}