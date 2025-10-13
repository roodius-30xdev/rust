/*
    trait => type / interface
*/


struct Rect {
    w:f32,
    h:f32
}

impl Shape for Rect { // shape impl for Rect
    fn area(&self) -> f32 {
        return self.w * self.h
    }
}

struct Circle {
    r:f32
}

impl Shape for  Circle {   // shape impl for Circle
    fn area(&self) -> f32 {
        return self.r * self.r
    }
 }

trait Shape {   //   [such a funcion need who return f32 valule]  its like a type decleration of rs function like [d.ts]
    fn area(&self) -> f32;  // Shape of your final Thing
}

fn print_area_of_shape<T:Shape>(s:T){
    println!("{}", s.area());
 }

fn main(){
    let r =  Rect {
        w:10.2,
        h:23.9
    };

    let c = Circle {
        r:21.2
    };

    print_area_of_shape(c);

}