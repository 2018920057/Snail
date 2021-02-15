use std::io;

static mut day: i32 = 0;
static mut meter: i32 = 0;

fn main() {
  unsafe{
  let mut input: String = String::new();
    println!("며칠동안?");
    io::stdin().read_line(&mut input).expect("input error");
    day = input.trim().parse().expect("parsing error");
    input.clear();
    println!("몇미터?");
    io::stdin().read_line(&mut input).expect("input error");
    meter = input.trim().parse().expect("parsing error");
    input.clear();
    print!("달팽이가 {}일동안 {}미터를 올라갈 수 있는 경우의 수는 {}가지 입니다.", day, meter, climb(0,0));
  }
}


//달팽이가 days일 동안 climbed미터를 기어올라 왔을 때,
//days일 전까지 meter미터를 올라갈 수 있는 경우의 수
fn climb(days: i32, climbed: i32) -> i32 {
  //TODO
  0
}