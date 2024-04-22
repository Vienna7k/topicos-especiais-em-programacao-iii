use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};


#[derive(Debug)]
pub enum Status {
    Disponivel,
    Indisponivel,
}

#[derive(Debug)]
pub struct Produto {
    pub nome: String,
    pub qtd: i32, //quantidade no estoque
    pub status: Status,
}

pub fn ler_produtos(arquivo: &str) -> io::Result<Vec<Produto>> {
    let file = File::open(arquivo)?;
    let reader = BufReader::new(file);
    let mut produtos = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let info: Vec<&str> = line.split(';').collect();
        let nome = info[0].to_string();
        let qtd: i32 = info[1].parse().expect("Erro ao converter quantidade");
        let status = if qtd > 0 { Status::Disponivel } else { Status::Indisponivel };

        let produto = Produto { nome, qtd, status };
        produtos.push(produto);
    }

    Ok(produtos)
}

pub fn atualizar_arquivo_produtos(arquivo: &str, produtos: &[Produto]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(arquivo)?;

    for produto in produtos {
        let status_str = match produto.status {
            Status::Disponivel => "disponível",
            Status::Indisponivel => "indisponível",
        };

        writeln!(file, "{};{};{}", produto.nome, produto.qtd, status_str)?;
    }

    Ok(())
}


pub fn read_index(produtos: &[Produto]) -> io::Result<usize> {
    let mut escolha = String::new();
    io::stdin()
        .read_line(&mut escolha)
        .expect("Falha ao ler entrada");
    let index = escolha.trim().parse::<usize>().unwrap() - 1;

    if index >= produtos.len() {
        panic!("Índice inválido");
    }

    Ok(index)
}

pub fn read_quantidade(prompt: &str) -> io::Result<i32> {
    println!("{}", prompt);

    let mut quantidade_str = String::new();
    io::stdin()
        .read_line(&mut quantidade_str)
        .expect("Falha ao ler entrada");
    let quantidade = quantidade_str.trim().parse::<i32>().expect("Erro ao converter quantidade");

    Ok(quantidade)
}
