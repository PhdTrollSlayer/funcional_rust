mod corrida_func;

fn main() {
    let arr = vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0];

    // Rust é por padrão preguiçoso. Isto é, ele só vai fazer os calculos se os valores forem utilizados de alguma forma
    // Por isso o uso do collect()
    let impostos: Vec<f32> = arr.iter().map(|ele| calcular_impostos(ele)).collect();

    dbg!(impostos);

    // Exemplo de lambda (No rust chamada de funções anônimas)
    let pares: Vec<f32> = arr
        .iter()
        .cloned()
        .filter(|ele| (*ele % 2.0) == 0.0)
        .collect();

    dbg!(pares);

    corrida_func::corrida(&mut vec![2, 6]);
}

fn calcular_impostos(x: &f32) -> f32 {
    x * 0.4
}
