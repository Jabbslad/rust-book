#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );

    let rect = (30, 50);
    println!("The area of the rectangle is {} square pixels", area2(rect));
    println!("{:?}", rect);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    let area = area3(&rect2);
    println!("The area of the rectangle is {} square pixels", area);
    println!("{:#?}", rect2);

    let area = Rectangle {
        width: 30,
        height: 50,
    }
    .area();
    println!("The area of the rectangle is {} square pixels", area);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("{:#?}", Rectangle::square(10));
}

fn area(w: u32, h: u32) -> u32 {
    w * h
}
fn area2(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}
fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn area3_is_1500() {
        assert_eq!(
            1500,
            area3(&Rectangle {
                width: 30,
                height: 50
            })
        );
    }

    #[test]
    fn rect_area_is_1500() {
        assert_eq!(
            1500,
            Rectangle {
                width: 30,
                height: 50
            }
            .area()
        );
    }
}
