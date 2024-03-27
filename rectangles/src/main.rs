struct Rectanle{
    width:u32,
    height:u32
}

fn main() {
    // println!("Hello, world!");
    let rec=Rectanle{
        width: 30,
        height: 40
    };
    println!("{}",area(&rec))

}

fn area(rect:&Rectanle)->u32{
    rect.width * rect.height
}
