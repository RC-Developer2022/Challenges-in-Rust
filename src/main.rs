mod desafios;

fn main() {
    match desafios::fibonacci_desafio::fibonacci(5){
        Ok(res) =>{
            println!("{:?}",res);
        }
        Err(erro)=> {
            eprintln!("erro: {}", erro);
        }
    }
    match desafios::fibonacci_desafio::fibonacci(0){
        Ok(res) =>{
            println!("{:?}",res);
        }
        Err(erro)=> {
            eprintln!("erro: {}", erro);
        }
    }
    match desafios::fibonacci_desafio::fibonacci(-5){
        Ok(res) =>{
            println!("{:?}",res);
        }
        Err(erro)=> {
            eprintln!("erro: {}", erro);
        }
    }
}
