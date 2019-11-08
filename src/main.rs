mod corrida_func;

fn main() {
    let arr = vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0];

    // Rust é por padrão preguiçoso. Isto é, ele só vai fazer os calculos se os valores forem utilizados de alguma forma
    // Por isso o uso do collect()
    let impostos: Vec<f32> = arr.iter().map(|ele| calcular_impostos(ele)).collect();

    // Exemplo de lambda (No rust chamada de funções anônimas)
    let pares: Vec<f32> = arr
        .iter()
        .cloned()
        .filter(|ele| (*ele % 2.0) == 0.0)
        .collect();

    // Duas formas de criar funções
    // Fazem a mesma coisa
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    // Função anonima (closure)
    let add_one_v2 = |x: u32| -> u32 { x + 1 };

    // Closure sem definição de tipo de entrada
    let plus_two = |x| {
        let mut result: i32 = x;

        result += 2;

        result
    };

    // Exemplo onde a closure captura para si a propriedade de num
    // Utilizado onde existe uma thread separada que precisa manter uma cópia dos valores na
    // propria thread
    let mut num = 5;
    {
        let mut add_num = move |x: i32| num += x; // Se tirarmos o 'move' a closure não vai capturar o ambiente e 'num' vai ter o valor 10

        add_num(5);
    }
    // num == 5

    // Exemplo passando função como parâmetro
    let ret = recebe_func(|x| x * 2, 6);

    // Exemplo recebendo uma função como retorno
    let divide_por_dois = retorna_func(2.0);
    let result = divide_por_dois(32.0);

    // Exemplo simples
    corrida_func::corrida(&mut vec![2, 6]);
}

fn recebe_func<F>(func: F, arg: i16) -> i16
where
    F: Fn(i16) -> i16,
{
    func(arg)
}

fn retorna_func(divisor: f32) -> Box<dyn Fn(f32) -> f32> {
    Box::new(move |x| x / divisor)
}

fn calcular_impostos(x: &f32) -> f32 {
    x * 0.4
}
