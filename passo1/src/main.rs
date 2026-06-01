fn main() {
    // String imutável, por padrão todas as variáveis em Rust são imutaveis.
    let texto = String::from("O gato roeu a roma do rei de Roma, o gato sumiu");
    
    // Convesão do texto para minusculo.
    let texto_minusculo = texto.to_lowercase();

    // Vetor com todas as palavras contidas no texto.
    // As referencias são posições em memória, se `texto_minusculo` mudar, o vetor irá quebrar (Borrow Checker)
    // Não posso recuperar uma informação de uma variável temporária.
    let palavras: Vec<&str> = texto_minusculo.split_whitespace().collect();

    // Apresentação dos dados
    println!("Texto tokenizado: {:?}", palavras);
}
