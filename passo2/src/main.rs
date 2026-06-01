use std::collections::HashMap;

/*
Ciclo de Vida (Lifetimes): 
Perceba que modelo guarda chaves do tipo &str (referências). 
De onde vêm essas referências? Da variável texto_minusculo. 
Se você tentar destruir ou alterar texto_minusculo antes do fim do programa, 
o compilador do Rust vai travar o código dizendo que o modelo está tentando 
apontar para uma memória que não existe mais. 
Isso evita os famosos NullPointerException do Java ou Segmentation Faults do C.

O operador asterisco (*contagem += 1): 
O método .or_insert(0) retorna uma referência mutável (&mut u32) para o número dentro do mapa. 
Para alterar o número real e não o ponteiro, precisamos "ir até o endereço de memória", 
o que fazemos com o operador *.
*/
fn main() {
    // Referente ao passo1, recupera o texto, converte para minuscula e tokeniza.
    let texto = String::from("O gato roeu a roupa do rei de Roma, o gato sumiu");
    let texto_minusculo = texto.to_lowercase();
    let palavras: Vec<&str> = texto_minusculo.split_whitespace().collect();

    // Este é o mapa de frequencia, o primeiro key é a palavra base, o segundo a palavra relacionada e o value é a quantidade de ocorrencias da palavra.
    // Esta variável é criada como mutável.
    let mut modelo: HashMap<&str, HashMap<&str, u32>> = HashMap::new();

    // O laço acontece recuperando dois valores simultaneos.
    // para o primeiro, recupera o primeiro e o segundo.
    // para o segundo, recupera o segundo e o terceiro.
    // e assim por diante.
    for par in palavras.windows(2) {
        let palavra_atual = par[0];
        let palavra_proxima = par[1];

        // Recupera o valor da palavra no map, caso não exista é incluida uma nova referencia
        let palavra_proxima_map = modelo.entry(palavra_atual).or_insert(HashMap::new());

        // Recupera a contagem das palavra, caso não existe inicia com 0
        let contagem = palavra_proxima_map.entry(palavra_proxima).or_insert(0);

        // como contagem é um ponteiro, quando uso um asteristico no começo, altero o conteúdo, não a posição.
        *contagem += 1;
    }

    // Imprime a matriz de treinamento.
    println!("Modelo treinado:\n{:#?}", modelo);
}
