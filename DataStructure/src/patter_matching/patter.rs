



fn main(){
    let num = -5;
    
    match num {
        n if n > 0 => println!("Positive"),
        n if n < 0 => println!("negative"),
        _ => println!("zero")        
    }

    let num2 = 0;

    match num2 {
        1..=i32::MAX => println!("Positive number"),
        i32::MIN..=-1 => println!("Negative number"),
        0 => println!("Zero"),
    }

    let age  = 18;

    match age {
        n if n % 2 == 0 => println!("Able to vote"),
        n if n % 2 != 0 => println!("under age"),
        _ => println!("unknown number")
    }

    let number = 3;

    match number {
        1 | 2 => println!("one or two not three"),
        3 | 4 |5 => println!("three,Four,five"),
        _ => println!("Something else")
    }

    let dir = Dir::Up;
    
    match dir {
        Dir::Up | Dir::Down => println!("Vertical Directions"),
        Dir::Left | Dir::Right => println!("Horizontal Direction"),
    }

    let score = 98;

    match score {
        90..=100 => println!("grade A"),
        80..=89 => println!("Grade B"),
        70..=79 => println!("Grade C"),
        _ => println!("Grade: D and F")
    }

    let ch = 'g';

    match ch {
        'a'..='z' => println!("Lowercase Character"),
        'A'..='Z' => println!("uppercase character"),
        _ => println!("not a character")
    }

    // Option inside another option

    let value = Some(Some(10));

    let data = value.unwrap().unwrap();
    println!("{}", data);

    match value {
        Some(Some(num)) => println!("Double Some with value: {}", num),
        Some(None) => println!("Outer Some inner None"),
        _ => println!("completley Empty")        
    }

    // nested enum
    let shape = Some(Shape::Rectangle { widht: 34, height: 45 });

    match shape {
        Some(Shape::Rectangle { widht, height }) => println!("Rectangle area w * h = {}",widht*height),
        Some(Shape::Cicle(r)) => println!("Cicle with Radius: {}",r),
        None => println!("no shape here")
    }


}

enum Shape {
    Rectangle {widht:u32,height:u32},
    Cicle(u32)   
}

enum Dir {
    Up,
    Down,
    Left,
    Right
}
