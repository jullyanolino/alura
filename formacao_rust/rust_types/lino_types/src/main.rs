fn matriz() {
    let matriz:[[f32; 3]; 4] = [
        [1.0; 3],
        [1.2; 3],
        [1.4; 3],
        [1.6; 3],
    ];

    for linha in matriz {
        for item in linha {
            println!("Item {}", item);
        }
    }
}

enum DayOfWeek {
    Domingo,
    Segunda,
    Terça,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

fn is_weekend(day_of_week: DayOfWeek) -> bool {

    match day_of_week {
        DayOfWeek::Domingo | DayOfWeek::Sabado => true,
        _ => false
    }
}

#[allow(dead_code)]
enum Color {
    Red, 
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor{cyan: u8,magenta: u8,yellow: u8,black: u8}
}

fn cores() {
    let _color_rgb: Color = Color::RgbColor(0,0,0);
    let color_cymk: Color = Color::CymkColor{cyan: 100,magenta: 50,yellow: 70,black: 255};

    println!("Cor {}", match color_cymk {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "azul",
        Color::RgbColor(0, 0, 0) 
            | Color::CymkColor{cyan: _,magenta: _,yellow: _,black: 255}=> "preta",
        Color::RgbColor(_, _, _) => "RGB desconhecido",
        Color::CymkColor{cyan: _,magenta: _,yellow: _,black: _} => "CYMK desconhecido"
    });
}

fn conteudo_opcional() {
    let conteudo:Option<String> = ler_arquivo(String::from(""));

    match &conteudo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo não existe")
    };

    println!("{:?}", &conteudo);

    if let Some(valor) = &conteudo {
        println!("Agora tenho certeza de há o valor {}", valor);
    }

}



enum Result<S, E> {
    Ok(S),
    Err(E)
}

fn ler_arquivo(caminho: String) -> Option<String> {
    Some(String::from("Conteúdo do Arquivo"))

}

fn vectors() {
    let mut notas:Vec<f32> = Vec::new();
    notas.push(10.0);
    notas.push(13.0);
    notas.push(11.0);
    println!("{:?}", notas);
    println!("Capacidade: {}", notas.capacity());

    let mut notas_prox:Vec<f32> = vec![10.0, 13.0, 11.0];
    notas_prox.push(5.5);
    println!("{:?}", notas_prox);
    println!("Capacidade: {}", notas_prox.capacity());

    let mut notas_:Vec<f32> = Vec::with_capacity(4);
    println!("Capacidade: {}", notas_.capacity());

    println!("Nota 1: {}", notas[0]);

    println!("Nota 6: {}", match notas_prox.get(7) {
        Some(n) => *n,
        None => -1.0
    });
/*
    while let Some(nota) = notas.pop() {
        println!("Valor removido: {}", nota);
    }
 */

    for nota in &notas {
        println!("Nota: {}", nota);
    }


    println!("Notas: {:?}", notas);
}

struct Titular {
    nome: String,
    sobrenome: String
}

struct Conta {
    titular: Titular,
    saldo: f64
}

impl Conta{
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;
    }
}

fn conta_corrente() {
    let titular_: String = String::from("Jullyano Lino");
    let saldo_: f64 = 100.0;

    let titular: Titular = Titular{nome: String::from("Jullyano"), sobrenome: String::from("Lino")};
    let mut conta_1: Conta = Conta{
        titular, 
        saldo: 100.0
    };

    conta_1.sacar(50.0);

    println!(
        "Dados da conta. Nome: {}, Sobrenome: {} / Saldo:{}", 
        conta_1.titular.nome, 
        conta_1.titular.sobrenome, 
        conta_1.saldo
    );


}


fn main() {
    let notas: [f32; 4] = [10.0, 1.0, 9.5, 6.0];
    let _notas_novas: [f32; 4] = [10.0; 4];
    let inteiro: usize = 0;

    println!("{}", notas[inteiro]);

    for nota in notas {
        println!("A nota é: {}", nota);
    }

    for i in 0..notas.len() {
        println!("A nota {} é {}", i + 1, notas[i]);
    }

    matriz();


    let dia: DayOfWeek = DayOfWeek::Sexta;
    println!("É fim de semana? {}", is_weekend(dia));

    cores();

    conteudo_opcional();

    vectors();

    conta_corrente();
}

