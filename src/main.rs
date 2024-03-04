mod desafios;

fn main() {
    match desafios::merge_string_alternately_leetcode_challengers::merge_alternately("ab","pqrs"){
        Ok(res) => {
            println!("{}",res)
        }
        Err(erro)=>{

        }
    }
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