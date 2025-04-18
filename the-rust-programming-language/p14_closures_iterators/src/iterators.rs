pub fn call1() {
    let v1: Vec<i32> = vec![1, 2, 3];
    for val in v1.iter() {
        println!("값: {val}");
    }

    let v1_iter: std::slice::Iter<'_, i32> = v1.iter();
    for val in v1_iter {
        println!("값: {val}");
    }
}

#[test]
fn iterator_demonstration() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let mut v1_iter: std::slice::Iter<'_, i32> = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter: std::slice::Iter<'_, i32> = v1.iter();
    let total: i32 = v1_iter.sum();

    // for val in v1_iter {   // ERROR
    //   println!("값: {}", val);
    // }

    assert_eq!(total, 6);
}

#[test]
pub fn call2() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x: &i32| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size:  u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|s: &Shoe| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes: Vec<Shoe> = vec![
        Shoe {
            size:  10,
            style: String::from("스니커즈"),
        },
        Shoe {
            size:  13,
            style: String::from("샌달"),
        },
        Shoe {
            size:  10,
            style: String::from("부츠"),
        },
    ];

    let in_my_size: Vec<Shoe> = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size:  10,
                style: String::from("스니커즈"),
            },
            Shoe {
                size:  10,
                style: String::from("부츠"),
            },
        ]
    );
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter: Counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x: &u32| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
