// enum Option<T> {
//     None,
//     Some(T),
// }

pub fn options() -> Result<(), Box<dyn std::error::Error>> {
    let original: Option<i32> = Some(5);

    println!("-------------------------------------");

    // Case 1: map()
    let result1 = original.map(|value| value * 2);
    println!("Original: {:?}", original);
    println!("Result_1: {:?}", result1);

    // Case 2: and_then()
    let result2 = original.and_then(|value| Some(value.to_string()));
    println!("Original: {:?}", original);
    println!("Result_2: {:?}", result2);

    // Case 3: unwrap_or()
    let result3 = original.unwrap_or(0);
    println!("Original: {:?}", original);
    println!("Result_3: {:?}", result3);

    // Case 4: unwrap_or_else()
    let result4 = original.unwrap_or_else(|| 0);
    println!("Original: {:?}", original);
    println!("Result_4: {:?}", result4);

    Ok(())
}
