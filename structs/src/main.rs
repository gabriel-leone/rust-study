fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    let rect3 = Rectangle {
        width: 25,
        height: 45,
    };

    // println!("rect3 is {:#?}", rect3);

    dbg!(&rect3);

    println!(
        "Can rect2 hold rect3? {}",
        rect2.can_hold(&rect3)
    );

    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    dbg!(&rect4);

    println!(
        "Can rect2 hold rect4? {}",
        rect2.can_hold(&rect4)
    );

    let sq = Rectangle::square(3);
    dbg!(&sq);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
