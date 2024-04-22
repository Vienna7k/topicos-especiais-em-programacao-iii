
use funcoes::{Produto, Status, ler_produtos,atualizar_arquivo_produtos, read_index, read_quantidade};
use std::io;

fn main() -> io::Result<()> {
    let arquivo = "produtos.txt";
    let mut produtos = ler_produtos(arquivo)?;

    loop {
        println!("Escolha uma opção:");
        println!("1 - Visualizar informações gerais dos produtos");
        println!("2 - Adicionar produtos ao estoque");
        println!("3 - Diminuir produtos do estoque");
        println!("4 - Sair");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha na entrada");

        match input.trim() {
            "1" => {
                for produto in &produtos {
                    println!("{:?}", produto);
                }
            }
            "2" => {
                println!("Escolha o produto para adicionar ao estoque:");
                for (index, produto) in produtos.iter().enumerate() {
                    println!("{} - {}", index + 1, produto.nome);
                }

                let index = read_index(&produtos)?;
                let quantidade = read_quantidade("Digite a quantidade a ser adicionada:")?;

                produtos[index].qtd += quantidade;

                // Atualizar o status para "Disponível" se a quantidade for maior que 0
                if produtos[index].qtd > 0 {
                    produtos[index].status = Status::Disponivel;
                }

                atualizar_arquivo_produtos(arquivo, &produtos)?;
            }
            "3" => {
                println!("Escolha o produto para diminuir do estoque:");
                for (index, produto) in produtos.iter().enumerate() {
                    println!("{} - {}", index + 1, produto.nome);
                }

                let index = read_index(&produtos)?;
                let quantidade = read_quantidade("Digite a quantidade a ser diminuída:")?;

                if quantidade > produtos[index].qtd {
                    println!("Quantidade solicitada é maior do que a quantidade disponível!");
                    continue;  // Volta para o início do loop
                }

                produtos[index].qtd -= quantidade;

                // Atualizar o status para "indisponível" se a quantidade for 0
                if produtos[index].qtd == 0 {
                    produtos[index].status = Status::Indisponivel;
                }


                atualizar_arquivo_produtos(arquivo, &produtos)?;
            }
            "4" => break,
            _ => println!("Opção inválida"),
        }
    }

    Ok(())
}
