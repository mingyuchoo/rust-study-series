// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

pub fn results() -> Result<(), Box<dyn std::error::Error>> {
    let original: Result<i32, &str> = Ok(3);

    println!("-------------------------------------");

    // Case 1: ? operator
    let result1 = original?;
    println!("Original: {:?}", original);
    println!("Result_1: {:?}", result1);

    // Case 2: unwrap
    let result2 = original.unwrap();
    println!("Original: {:?}", original);
    println!("Result_2: {:?}", result2);

    // Case 3: unwrap_or
    let result3 = original.unwrap_or(0);
    println!("Original: {:?}", original);
    println!("Result_3: {:?}", result3);

    // Case 4: unwrap_or_default
    let result4 = original.unwrap_or_default();
    println!("Original: {:?}", original);
    println!("Result_4: {:?}", result4);

    // Case 5: unwrap_or_else
    let result5 = original.unwrap_or_else(|_| -1);
    println!("Original: {:?}", original);
    println!("Result_5: {:?}", result5);

    // Case 6: unwrap_err
    let result6 = original.unwrap_err();
    println!("Original: {:?}", original);
    println!("Result_6: {:?}", result6);

    Ok(())
}
