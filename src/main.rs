mod desafios;

fn main() {
    result_fibonacci(5,0,-5)
}

fn result_fibonacci(num: i32, num1:i32, num2: i32) {
    match desafios::fibonacci_desafio::fibonacci(num){
        Ok(res) =>{
            println!("{:?}",res);
        }
        Err(erro)=> {
            eprintln!("erro: {}", erro);
        }
    }
    match desafios::fibonacci_desafio::fibonacci(num1){
        Ok(res) =>{
            println!("{:?}",res);
        }
        Err(erro)=> {
            eprintln!("erro: {}", erro);
        }
    }
    match desafios::fibonacci_desafio::fibonacci(num2){
        Ok(res) =>{
            println!("{:?}",res);
        }
        Err(erro)=> {
            eprintln!("erro: {}", erro);
        }
    }
}