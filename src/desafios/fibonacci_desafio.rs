pub fn fibonacci(num: i32) -> Result<Vec<i32>, &'static str> {

    if num < 0 {
        return Err("Num needs to be a postive integer");
    }

    let mut response: Vec<i32> = Vec::new();

    let mut prev: i32 = 0;
    let mut curr: i32 = 1;

    for _i in 0..num {
        response.push(curr);
        curr = curr+ prev;
        prev = curr;
    }
    return Ok(response);
}