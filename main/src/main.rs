pub mod shape {
    pub struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        pub fn new(w: u32, h: u32) -> Self {
           Self { width: w, height: h }
        }
        pub fn width(&self) -> u32 {
           self.width
        }
        pub fn height(&self) -> u32 {
           self.height
        }
    }

}

use shape::Rectangle;
use pedas::sync::list::List;
use pedas::list;

impl Rectangle {
    fn area(&self) -> u32 {
        self.width() * self.height()
    }
    fn area2(&self, q: u32) -> u32 {
        q * self.width() * self.height()
    }
}

fn area3(r: &Rectangle, q: u32) -> u32 {
    q * r.width() * r.height()
}

fn main() {
    //let ls = List::new(&[1, 4, 5, 9, 3]);
    let ls = list!(1, 4, 5, 9, 3);
    println!("{:?}", ls);

    let empty_list: List<i32> = List::empty();
    let one_item_list = empty_list.add(7);
    let _two_item_list = one_item_list.add(9);
    let list = one_item_list
        .add(5)
        .add(8)
        .add(4)
        .add(2)
        .add(8)
        .add(6)
        .add(10)
        .add(2)
        .add(7)
    ;
    println!("{:?}", list);

    let list_double = list.map(|x| x * 2);
    println!("{:?}", list_double);

    let rect1 = Rectangle::new(3, 2);
    let rect2 = Rectangle::new(4, 5);
    let rect3 = Rectangle::new(10, 20);

    println!("Area of rect1: {}", rect1.area());
    println!("Area of rect2: {}", rect2.area());
    println!("Area of rect3: {}", rect3.area());
    println!("Area of rect3: {}", Rectangle::area(&rect3));
    println!("Area of rect3: {}", rect3.area2(3));
    println!("Area of rect3: {}", Rectangle::area2(&rect3, 3));
    println!("Area of rect3: {}", area3(&rect3, 3));

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
