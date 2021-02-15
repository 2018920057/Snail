use std::io;
use std::collections::HashMap;

static mut day: i32 = 0;
static mut meter: i32 = 0;
static mut p: f64 = 0.0;
//달팽이가 우물벽을 타고 올라갈 때,
//1미터 또는 2미터를 올라간다.
fn main() {
  unsafe{
  let mut input: String = String::new();
    println!("며칠동안?");
    io::stdin().read_line(&mut input).expect("input error");
    day = input.trim().parse().expect("parsing error");
    p = 2.0_f64.powf(day as f64);
    input.clear();
    println!("몇미터?");
    io::stdin().read_line(&mut input).expect("input error");
    meter = input.trim().parse().expect("parsing error");
    input.clear();
    let mut cache: HashMap<(i32,i32),i32> = HashMap::new();
    println!("(50:50) 달팽이가 {}일동안 {}미터를 올라갈 수 있는 확률은 {}% 입니다.", day, meter, (climb(0,0,&mut cache) as f64/p)*100.0);
    let mut cache2: HashMap<(i32,i32),f64> = HashMap::new();
    println!("(25:75) 달팽이가 {}일동안 {}미터를 올라갈 수 있는 확률은 {}% 입니다.", day, meter, climb2(0,0,&mut cache2)*100.0);
  }
}


//달팽이가 days일 동안 climbed미터를 기어올라 왔을 때,
//day일 전까지 meter미터를 올라갈 수 있는 경우의 수
//1미터인지 2미터인지의 확률은 반반.
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

//1미터인지 2미터인지의 확률은 25:75.
unsafe fn climb2(days: i32, climbed: i32, cache: &mut HashMap<(i32,i32),f64>) -> f64 {
  //기저 사례: day일이 지났을 때
  if days==day {
    if climbed>=meter {return 1.0;}
    else {return 0.0;}
  }
  //memoization
  let ret: f64 = match cache.get(&(days,climbed)){
    Some(_) => 0.0,
    None => 0.25*climb2(days+1,climbed+1,cache)+0.75*climb2(days+1,climbed+2,cache),
  };
  *cache.entry((days,climbed)).or_insert(ret)
}