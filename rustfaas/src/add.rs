#[no_mangle]
pub extern "C" fn invoke(input: String) -> Result<String, String> {
    let numbers: Result<Vec<i32>, _> = input.split(',')
        .map(|s| s.trim().parse())
        .collect();

    match numbers {
        Ok(numbers) => {
            let sum: i32 = numbers.iter().sum();
            Ok(sum.to_string())
        },
        Err(e) => Err(format!("Failed to parse numbers: {}", e)),
    }
}
