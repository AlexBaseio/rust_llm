# Rust LLM
Iniciado um trabalho de aprendizado tanto em Rust como em LLM.

Vou incluir aqui o passo a passo que iniciei com o Gemini para construir uma solução simples de LLM.

Estou usando o Arch Linux (WSL) com Rust instalado.
Não vou incluir como instalar o Arch Linux, nem o Rust (Isso é muito fácil de achar de fazer).


Entendo perfeitamente o seu desconforto. Quando geramos blocos longos de texto explicativo misturados com múltiplos blocos de código, interfaces de chat às vezes sofrem com limites invisíveis de caracteres ou truncam a resposta para economizar processamento. Isso não é uma desculpa, é apenas a realidade técnica de trabalhar com textos massivos.

Para resolver isso de forma definitiva e sem cortes, montei um **documento unificado e completo**. Ele traz absolutamente tudo o que discutimos até agora: a matemática dos modelos probabilísticos, o choque cultural da memória, a análise detalhada dos erros que você vivenciou e os códigos funcionais do **Passo 1** e do **Passo 2**.

Você pode copiar o bloco de código Markdown abaixo na íntegra e salvá-lo como `README.md` no seu projeto do Arch Linux:

---

```markdown
# 🦀 Aprendizado Duplo: Mini-LLM Probabilístico do Zero em Rust

[cite_start]Este repositório documenta o desenvolvimento passo a passo de um modelo probabilístico de linguagem natural para previsão de próxima palavra (similar aos teclados de smartphones)[cite: 3]. [cite_start]O projeto é construído em **Rust**, utilizando puramente a biblioteca padrão e sem o auxílio de dependências externas (*zero dependencies*), com o objetivo de fixar os conceitos matemáticos de Inteligência Artificial e as regras lógicas de gerenciamento de memória de baixo nível[cite: 4].

---

## 📚 Índice Analítico
1. [Conceituação Teórica: Modelos de N-Gramas (Bigramas)](#1-conceituação-teórica-modelos-de-n-gramas-bigramas)
2. [Passo 1: Tokenização de Strings e o Sistema de Posse (Ownership)](#2-passo-1-tokenização-de-strings-e-o-sistema-de-posse-ownership)
   - [Análise do Erro do Valor Temporário (E0716)](#análise-do-erro-do-valor-temporário-e0716)
   - [Mapeamento Físico de Memória: String vs &str](#mapeamento-físico-de-memória-string-vs-str)
   - [O Princípio da Imutabilidade por Padrão](#o-princípio-da-imutabilidade-por-padrão)
3. [Passo 2: Construindo a Matriz Probabilística (Tabelas de Frequência)](#3-passo-2-construindo-a-matriz-probabilística-tabelas-de-frequência)
   - [Código de Treinamento Completo](#código-de-treinamento-completo)
   - [Lógica do Mecanismo Entry e Desreferenciação Asterisco (*)](#lógica-do-mecanismo-entry-e-desreferenciação-asterisco-)
4. [Próximos Passos: O Motor de Inferência (Passo 3)](#4-próximos-passos-o-motor-de-inferência-passo-3)

---

## 1. Conceituação Teórica: Modelos de N-Gramas (Bigramas)

[cite_start]Modelos estatísticos de linguagem predizem a ocorrência de uma palavra analisando o histórico de termos que a precedem[cite: 12]. [cite_start]No modelo de **Bigrama**, assumimos a propriedade de Markov, onde a previsão depende unicamente da palavra imediatamente anterior[cite: 10, 12].

Se o nosso modelo for treinado com o corpus:  
[cite_start]`"o gato roeu a roupa do rei de roma o gato sumiu"` [cite: 13]

[cite_start]Ao receber o token `"o"`, o sistema calcula qual a maior probabilidade de transição empírica baseando-se no histórico de contagens[cite: 12, 13]:
* [cite_start]Par `("o", "gato")`: Ocorreu 2 vezes[cite: 14].
* [cite_start]Par `("o", "rei")`: Ocorreu 0 vezes (o token `"rei"` é precedido por `"do"`)[cite: 14].

A probabilidade condicional matemática é calculada pela fórmula:

$$P(\text{palavra}_2 \mid \text{palavra}_1) = \frac{\text{Contagem}(\text{palavra}_1, \text{palavra}_2)}{\text{Contagem}(\text{palavra}_1)}$$

---

## 2. Passo 1: Tokenização de Strings e o Sistema de Posse (Ownership)

[cite_start]O primeiro estágio de processamento de linguagem natural é a **Tokenização**, que consiste em segmentar a cadeia contínua de texto em um vetor de palavras[cite: 17]. 

### Análise do Erro do Valor Temporário (E0716)
[cite_start]Ao tentar encadear o processamento de texto de forma linear em uma única linha, o compilador do Rust impede a geração do executável emitindo o erro `temporary value dropped while borrowed`[cite: 53, 58]:

```rust
// CÓDIGO INCORRETO:
let palavras: Vec<&str> = texto.to_lowercase().split_whitespace().collect();

```

* 
**Por que ocorre?** O método `.to_lowercase()` gera uma nova `String` alocada dinamicamente na memória Heap. Como ela não foi explicitamente associada a uma variável estável, ela é um valor temporário limpo ao término da execução da linha (no ponto e vírgula `;`). O método `.split_whitespace()` não copia textos, ele apenas gera ponteiros de leitura (`&str`) que espiam a string interna. Sem a intervenção protetiva do compilador (o *Borrow Checker*), o vetor `palavras` conteria ponteiros para memória destruída, gerando um erro de *Segmentation Fault* em execução.


* 
**A Correção:** Isolar a criação do texto modificado em uma variável de escopo para estender seu ciclo de vida (*lifetime*).



### Código Corrigido de Tokenização:

```rust
fn main() {
    let texto = String::from("o gato roeu a roupa do rei de roma o gato sumiu");

    // Mantendo a nova alocação na Heap viva durante o escopo da função main
    let texto_minusculo = texto.to_lowercase();

    // Slices de strings colhidos com segurança de dentro do dado estável
    let palavras: Vec<&str> = texto_minusculo.split_whitespace().collect();

    println!("Texto tokenizado: {:?}", palavras);
}

```

### Mapeamento Físico de Memória: String vs &str

Rust gerencia strings através de tipos com responsabilidades físicas de hardware bem distintas:

| Tipo | Nome Comum | Localização Física na RAM | Características Principais |
| --- | --- | --- | --- |
| `String` | String Dinâmica | Alocada na **Heap** (Ponteiro na Stack) 

 | Flexível, expansível em tempo de execução, dona de seus dados.

 |
| `&str` | String Slice / Literal | Segmento de dados do binário (**ROData**) ou visão de terceiros 

 | Tamanho fixo, imutável, leitura ultrarrápida sem custos de alocação.

 |

> 
> **Nota de Arquitetura:** O método `.to_lowercase()` retorna uma `String` obrigatoriamente, visto que conversões de caixa podem alterar o peso em bytes UTF-8 de caracteres específicos, demandando uma estrutura maleável na memória Heap.
> 
> 

### O Princípio da Imutabilidade por Padrão

Variáveis declaradas via `let` são estritamente imutáveis no Rust. Isso impõe o Princípio do Menor Privilégio: se um dado funciona exclusivamente como base de leitura estática, ele é protegido contra mutações imprevistas. O relaxamento da mutabilidade através do `let mut` ocorre apenas sob necessidade estrita de modificação interna contínua.

---

## 3. Passo 2: Construindo a Matriz Probabilística (Tabelas de Frequência)

A tabela de contagens funciona como a inteligência bruta do modelo. Emulando o comportamento de dicionários aninhados presentes em linguagens interpretadas de alto nível (como `dict[str, dict[str, int]]` do Python) , criamos uma matriz multidimensional indexada por chaves textuais utilizando `std::collections::HashMap`.

### Código de Treinamento Completo

Substitua o conteúdo de `src/main.rs` pelo código estritamente tipado abaixo:

```rust
use std::collections::HashMap;

fn main() {
    let texto = String::from("o gato roeu a roupa do rei de roma o gato sumiu");
    
    let texto_minusculo = texto.to_lowercase();
    let palavras: Vec<&str> = texto_minusculo.split_whitespace().collect();

    // Estrutura de dados: HashMap< Palavra_Atual, HashMap< Proxima_Palavra, Contagem > >
    let mut modelo: HashMap<&str, HashMap<&str, u32>> = HashMap::new();

    // Análise por janelas deslizantes sequenciais de tamanho 2
    for par in palavras.windows(2) {
        let palavra_atual = par[0];
        let proxima_palavra = par[1];

        // Se a palavra_atual não existir nas chaves do modelo, inicializa um mapa interno vazio
        let proximas_palavras = modelo.entry(palavra_atual).or_insert(HashMap::new());

        // Acessa o contador correspondente à proxima_palavra, iniciando em 0 caso seja inédito
        let contagem = proximas_palavras.entry(proxima_palavra).or_insert(0);
        
        // Aplicação do operador de desreferenciação para alteração direta do valor numérico
        *contagem += 1; 
    }

    // Impressão estruturada (Pretty Print) do modelo matemático treinado
    println!("Modelo Estatístico Bruto Treinado:\n{:#?}", modelo);
}

```

### Lógica do Mecanismo Entry e Desreferenciação Asterisco (*)

A API do `HashMap` em Rust implementa o padrão `.entry()`, que realiza a busca e a inserção da chave de forma atômica e eficiente em memória.
O método `.or_insert(0)` nos devolve uma **referência mutável** (`&mut u32`) que aponta diretamente para o endereço físico do número armazenado dentro da tabela.

Como estamos manipulando um ponteiro, o código não pode simplesmente somar um valor ao endereço físico. O operador asterisco (`*`) atua desreferenciando o ponteiro, instruindo o processador a ir até o valor numérico bruto e adicionar `1` na posição real da memória RAM.

---

## 4. Próximos Passos: O Motor de Inferência (Passo 3)

Com a tabela de frequências armazenada estavelmente na memória do programa, o próximo passo lógico de desenvolvimento consiste em implementar a **Inferência do Modelo**. O algoritmo executará as seguintes subtarefas:

1. Receber uma palavra de entrada submetida pelo usuário no terminal.


2. Acessar a tabela correspondente no `HashMap` principal.


3. Computar a probabilidade das palavras candidatas dividindo suas contagens individuais pela soma total de ocorrências daquela linha.


4. Retornar ao terminal a palavra com a maior probabilidade preditiva calculada.



```

---

### Verificação de status do projeto
Copie o conteúdo acima e salve-o para garantir a sua documentação local. 

[cite_start]Agora que o conhecimento está centralizado e protegido, execute o comando `cargo run` com o código do Passo 2 na sua máquina virtual Arch Linux[cite: 41, 47]. [cite_start]O terminal exibiu com sucesso a árvore de chaves textuais e as contagens brutas semelhantes a isto[cite: 48]?

```text
Modelo Estatístico Bruto Treinado:
{
    "o": {
        "gato": 2,
    },
    "gato": {
        "roeu": 1,
        "sumiu": 1,
    },
    ...
}

```

Se tudo compilou perfeitamente e sem erros do *Borrow Checker*, estamos prontos para projetar a lógica matemática de inferência (escolha da próxima palavra)!
