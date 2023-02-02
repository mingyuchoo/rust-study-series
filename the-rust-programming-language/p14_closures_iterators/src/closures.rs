use std::thread;
use std::time::Duration;

pub fn call1() {
    let value = simulated_expensive_calculation(3);
    println!("{}", value);
}

pub fn call2() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout_1(simulated_user_specified_value, simulated_random_number);
}

pub fn call3() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout_2(simulated_user_specified_value, simulated_random_number);
}

pub fn call4() {
    let example_closure = |x| x;

    let s = example_closure(String::from("Hello"));
    println!("{}", s);

    // let n = example_closure(5);
    // println!("{}", n);
}

pub fn call5() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout_3(simulated_user_specified_value, simulated_random_number);
}

pub fn call6() {
    fn right() {
        let x = 4;
        let equal_to_x = |z| z == x;
        let y = 4;
        assert!(equal_to_x(y));
    }
    //fn wrong_1() {
    //    let x = 4;
    //    fn equal_to_x(x: i32) -> bool { z == x } // ERROR
    //    let y = 4;
    //    assert!(equal_to_x(y));
    //}

    // fn wrong_2() {
    //     let x= vec![1, 2, 3];
    //     let equal_to_x = move |z| z == x;
    //     println!("변수 x를 사용할 수 없습니다: {:?}", x); // ERROR
    //     let y = vec![1, 2, 3];
    //     assert!(equal_to_x(y));
    // }

    right();
}

pub fn call7() {}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("시간이 오래 걸리는 계산을 수행 중...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout_1(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("오늘은 {}번의 팔 굽혀펴기를 하세요!", expensive_result);
        println!("다음에는 {}번의 윗몸 일으키기를 하세요!", expensive_result);
    } else {
        if random_number == 3 {
            println!("오늘은 수분을 충분히 섭취하며 쉬세요!");
        } else {
            println!("오늘은 {}분간 달리기를 하세요!", expensive_result);
        }
    }
}

fn generate_workout_2(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("시간이 오래 걸리는 계산을 수행 중...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "오늘은 {}번의 팔 굽혀펴기를 하세요!",
            expensive_closure(intensity)
        );
        println!(
            "다음에는 {}번의 윗몸 일으키기를 하세요!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("오늘은 수분을 충분히 섭취하며 쉬세요!");
        } else {
            println!(
                "오늘은 {}분간 달리기를 하세요!",
                expensive_closure(intensity)
            );
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout_3(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num: u32| -> u32 {
        println!("시간이 오래 걸리는 계산을 수행 중...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "오늘은 {}번의 팔 굽혀펴기를 하세요!",
            expensive_result.value(intensity)
        );
        println!(
            "다음에는 {}번의 윗몸 일으키기를 하세요!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("오늘은 수분을 충분히 섭취하며 쉬세요!");
        } else {
            println!(
                "오늘은 {}분간 달리기를 하세요!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
