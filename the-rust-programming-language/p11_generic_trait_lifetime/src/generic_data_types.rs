pub fn call1() {
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("가장 큰 숫자: {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("가장 큰 문자: {result}");
}

// pub fn call2() {
//   fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list.iter() {
//       if item > largest {
//         largest = item;
//       }
//     }
//     largest
//   }
//
//   let number_list = vec![34, 50, 25, 100, 65];
//   let result = largest_i32(&number_list);
//   println!("가장 큰 숫자: {}", result);
//
//   let char_list= vec!['y', 'm', 'a', 'q'];
//   let result = largest_char(&char_list);
//   println!("가장 큰 문자: {}", result);
// }

pub fn call3() {
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10, };
    let float = Point { x: 1.0, y: 4.0, };

    println!("integer: ({},{}), float: ({},{})",
             integer.x, integer.y, float.x, float.y);
}

pub fn call4() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let both_integer = Point { x: 5, y: 10, };
    let both_float = Point { x: 1.0, y: 4.0, };
    let integer_and_float = Point { x: 5, y: 4.0, };

    print!("both_integer: ({},{}), ", both_integer.x, both_integer.y);
    print!("both_float: ({},{}), ", both_float.x, both_float.y);
    println!("integer_and_float: ({},{})",
             integer_and_float.y, integer_and_float.y);
}

pub fn call5() {
    struct Point<T> {
        x: T,
        y: T,
    }
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }

        fn y(&self) -> &T {
            &self.y
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x
                 .powi(2)
             + self.y
                   .powi(2)).sqrt()
        }
    }

    let p = Point { x: 5, y: 10, };
    let r = Point { x: 5.0, y: 10.0, };

    println!("p.x = {}, p.y = {}", p.x(), p.y());
    // println!("p.distance_from_origin = {}", p.distance_from_origin()); // CAN NOT
    // use this method for i32

    println!("r.x = {}, r.y = {}", r.x(), r.y());
    println!("r.distance_from_origin = {}", r.distance_from_origin());
}

pub fn call6() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self,
                       other: Point<V, W>)
                       -> Point<T, W> {
            Point { x: self.x,
                    y: other.y, }
        }
    }

    let p1 = Point { x: 5, y: 10.4, };
    let p2 = Point { x: "Hello", y: 'c', };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
