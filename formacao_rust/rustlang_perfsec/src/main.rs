const PI:f32 = 3.14;

static mut GLOBAL:f32 = 2.24;

fn soma (a:i32, b:i32) -> i32
{
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn sombra(){
    let a = 123;
    let b = 098;

    {
        let b = 321;
        println!("Variável b interna: {}", b);
    }

    println!("Variável a  externa: {}", a);
    println!("Variável b externa: {}", b);
 
}

fn escopo(){

    let _my_str:&'static str = "nome";

    
    println!("PI: {}", PI);
    unsafe{
        println!("Variável Global: {}", GLOBAL);
    }


    let variavel = 3000;
    println!("Variável: {} / Tamanho: {}", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("Decimal: {} / Tamanho: {}", decimal, std::mem::size_of_val(&decimal));

    let decimal_2:f32 = 2.7;
    println!("Decimal: {} / Tamanho: {}", decimal_2, std::mem::size_of_val(&decimal_2));


    let booleana = true;
    //booleana = true;
    println!("Booleana: {} / Tamanho: {}", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'H';
    println!("Tamanho: {}", std::mem::size_of_val(&letra));
 

}

fn condicionais (){
    let idade: u8 = 52;
    let autorizacao = true;
    let maioridade = idade >= 21;

    if maioridade {
        println!("Pode entrar");
    } else if idade > 17 && autorizacao{
        println!("Pode entrar com autorização");
    }else {
        println!("Não pode entrar");
    }

    let condicao = if maioridade { "maior" } else { "menor" };
    println!("É {} de idade universal", condicao);

    let linguagem = "Java";
    let proposito = match linguagem{
        "PHP" => "Web",
        _ => "Desconhecido"
    };

    println!("O propósito de linguagem {} é {}", linguagem, proposito);

}

fn loops() {
    let multiplicador:u8 = 5;
    let mut contador:u8 = 0;

    while contador < 10 {
        
        contador += 1;
        
        if contador == 5 {
            continue;
        }
        
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;
    loop{
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);

        if contador == 10{
            break;
        }
    }

    for i in 1..=10 {
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }

}

fn ownership(){
    let mut uma_string = String::from("Me");
    rouba(&mut uma_string);

    println!("[Ownership]string:\"{}\"/endereço: {}", uma_string,&uma_string);

}

fn rouba(string: &mut String){
    string.push_str(", here!");
    println!("[Rouba]string: \"{}\"/endereço: {}", string,&string);

}

fn pattern_matching(){
    for x in 1..=20{
        println!("{}: {}", x, match x{
            1 => "Pouco",
            2 | 3 => "Um pouquinho",
            4..=10 => "Um bocado",
            _ if x % 2 == 0 => "Uma boa quantidade",
            _ => "Muito"
        })
    }
}

fn erros(){
    //panic!("Erro proposital");
    match resultado(){
        Ok(s) => println!("Sucesso: {}", s),
        Err(numero) => println!("Código: {}", numero)
    }
}

fn resultado() -> Result<String, u8>
{
    //Ok(String::from("Tudo deu certo"))
    Err(41)
}

fn main(){
     
    //escopo();    

    //sombra();

    //println!("Soma de 2 + 2 = {}", soma(2,2));

    //condicionais();    
    
    //loops();

    //ownership();

    //pattern_matching();

    erros();

} 

