# Rust LLM
Iniciado um trabalho de aprendizado tanto em Rust como em LLM.

Vou incluir aqui o passo a passo que iniciei com o Gemini para construir uma solução simples de LLM.

Estou usando o Arch Linux (WSL) com Rust instalado.
Não vou incluir como instalar o Arch Linux, nem o Rust (Isso é muito fácil de achar de fazer).


# Construindo um Mini-LLM Probabilístico do Zero em Rust

Repositório dedicado ao aprendizado duplo e simultâneo da linguagem de programação **Rust** e dos fundamentos lógicos/matemáticos de **Modelos de Linguagem (LLMs)**. O objetivo final é construir um modelo probabilístico de previsão de próxima palavra (modelo de n-gramas) do zero, sem o uso de bibliotecas externas (*zero dependencies*).

---

## 📚 Sumário Histórico de Aprendizado

- [Passo 1: O Conceito do Modelo (N-Grams) e Tokenização](#passo-1-o-conceito-do-modelo-n-grams-e-tokeniza%C3%A7%C3%A3o)
- [Lições de Rust: Gerenciamento de Memória sob a Ótica de Strings](#li%C3%A7%C3%B5es-de-rust-gerenciamento-de-mem%C3%B3ria-sob-a-%C3%93tica-de-strings)
  - [O Erro do Valor Temporário (The Borrow Checker)](#o-erro-do-valor-tempor%C3%A1rio-the-borrow-checker)
  - [Imutabilidade por Padrão (`let` vs `let mut`)](#imutabilidade-por-padr%C3%A3o-let-vs-let-mut)
  - [Alocação Física: `String` vs `&str`](#aloca%C3%A7%C3%A3o-f%C3%ADsica-string-vs-str)
- [Próximos Passos (Passo 2: O Coração Estatístico)](#pr%C3%B3ximos-passos-passo-2-o-cora%C3%A7%C3%A3o-estat%C3%ADstico)

---

## Passo 1: O Conceito do Modelo (N-Grams) e Tokenização

O modelo de previsão do teclado do celular baseia-se em um **Modelo de N-Gramas** (especificamente, um *Bigrama*). Em vez de redes neurais profundas, ele utiliza probabilidade condicional estática para prever o próximo termo baseado na palavra atual.

### A Matemática por Trás
A probabilidade de uma palavra ocorrer dada a palavra anterior é calculada pela frequência com que aparecem juntas no texto de treino:

$$P(\text{palavra}_2 \mid \text{palavra}_1) = \frac{\text{Contagem}(\text{palavra}_1, \text{palavra}_2)}{\text{Contagem}(\text{palavra}_1)}$$

### Código Base para Tokenização
O processo inicial consiste em quebrar o texto bruto em unidades menores (*tokens*):

```rust
fn main() {
    let texto = String::from("o gato roeu a roupa do rei de roma o gato sumiu");

    // Mantendo a String viva para as referências espiatórias
    let texto_minusculo = texto.to_lowercase();

    // Criando referências extraídas por delimitador de espaços
    let palavras: Vec<&str> = texto_minusculo.split_whitespace().collect();

    println!("Texto tokenizado: {:?}", palavras);
}
