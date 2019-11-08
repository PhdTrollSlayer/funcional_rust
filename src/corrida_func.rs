use rand::Rng;

// Printa os carros na tela
pub fn printar(estado: &Vec<i16>) {
    let v: String = estado
        .iter()
        .map(|c| carro(*c))
        .map(|e| format! {"{}\n", e})
        .collect();

    print!("{}[2J", 27 as char);
    println!("{}", v);
}

// Gera um numero aleatorio
pub fn rng_dist() -> i16 {
    rand::thread_rng().gen_range(0, 8)
}

// Atualiza o estado e printa a cada turno
pub fn att_estado(estado: &mut Vec<i16>) {
    estado.iter_mut().for_each(|c| *c = rng_dist());
    printar(&estado);
}

// Preapara a String para printar
pub fn carro(dis: i16) -> String {
    let mut esp = String::new();

    for _ in 0..dis {
        esp.push('-');
    }

    esp.push_str("[=]");

    esp
}

// Loop da corrida
pub fn corrida(mut estado: &mut Vec<i16>) {
    loop {
        att_estado(&mut estado);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
