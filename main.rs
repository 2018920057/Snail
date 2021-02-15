use std::io;
use std::collections::HashMap;

static mut day: i32 = 0;
static mut meter: i32 = 0;

//달팽이가 우물벽을 타고 올라갈 때,
//1미터 또는 2미터를 올라간다(50%). 
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
    let mut cache: HashMap<(i32,i32),i32> = HashMap::new();
    let p: i32 = 2_i32.pow(day as u32);
    print!("달팽이가 {}일동안 {}미터를 올라갈 수 있는 확률은 {}% 입니다.", day, meter, (climb(0,0,&mut cache) as f64/p as f64)*100.0);
  }
}


//달팽이가 days일 동안 climbed미터를 기어올라 왔을 때,
//day일 전까지 meter미터를 올라갈 수 있는 경우의 수
unsafe fn climb(days: i32, climbed: i32, cache: &mut HashMap<(i32,i32),i32>) -> i32 {
  //기저 사례: day일이 지났을 때
  if days==day {
    if climbed>=meter {return 1;}
    else {return 0;}
  }
  //memoization
  let ret: i32 = match cache.get(&(days,climbed)){
    Some(_) => 0,
    None => climb(days+1,climbed+1,cache)+climb(days+1,climbed+2,cache),
  };
  *cache.entry((days,climbed)).or_insert(ret)
}