use std::io;
use std::str::FromStr;
use std::fs;

fn ler<T: FromStr>() -> T {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().ok().unwrap()
}

struct Tarefa {
    descricao: String,
    concluida: bool,
}

fn main() {
    
    let mut lista_tarefas: Vec<Tarefa> = Vec::new();

    if let Ok(conteudo) = fs::read_to_string("tarefas.txt"){
        for linha in conteudo.lines() {
            let partes: Vec<&str> = linha.splitn(2, ',').collect();
            if partes.len() == 2 {
                let concluida = partes[0] == "true";
                let descricao = partes[1].to_string();

                lista_tarefas.push(Tarefa { descricao, concluida });
            }
        }
    } 


    loop {
        println!("\n ---Gerenciador de tarefas---");
        println!("1. Adicionar tarefa");
        println!("2. Listar tarefa");
        println!("3. Sair");
        println!("4. Concluir tarefa");
        println!("Escolha uma opcao");

        let mut escolha: String = ler();

        match escolha.trim() {
            "1" => {
                println!("Digite uma descricao nova da tarefa");
                let mut nova_descricao: String = ler();

                let nova_tarefa = Tarefa {
                    descricao: nova_descricao.to_string(),
                    concluida: false,
                };

                lista_tarefas.push(nova_tarefa);
                println!("Inserção com sucesso");
                
            }
            "2" => {
                println!("\n --- TAREFAS ---");

                if lista_tarefas.is_empty(){
                    println!("Nenhuma tarefa por enquanto");
                }
                else{
                    for (i, tarefa) in lista_tarefas.iter().enumerate() {
                        let status = if tarefa.concluida { "[X]" } else 
                        { "[ ]" };
                        
                        println!("{}. {} {}", i + 1, status, tarefa.descricao);
                    }
                }
            }
            "3" => {
                println!("Salvando arquivo");
                let mut texto_para_salvar = String::new();

                for tarefa in &lista_tarefas {
                    texto_para_salvar.push_str(&format!("{},{}\n", tarefa.concluida, tarefa.descricao));
                }

                fs::write("tarefas.txt", texto_para_salvar).unwrap();
                println!("Salvo. Saindo...");
                return;
            }
            "4" => {
                println!("Digite o numero da tarefa que voce quer concluir");
                let indice: usize = ler();
                if indice > 0 && indice <= lista_tarefas.len(){
                    lista_tarefas[indice - 1].concluida = true;
                    println!("Concluido tarefa {} concluido.", indice);

                }
                else{
                    println!("Invalido, digite um indice existente");
                }
            }

            "5" => {
                println!("Digite o numero da tarefa que voce quer deletar:");
                let indice: usize = ler();

                if indice > 0 && indice <= lista_tarefas.len() {
                    lista_tarefas.remove(indice - 1);

                    println!("Deletado");
                }
                else{
                    println!("Invalido, digite um indice existente");
                }
            }

            _ => {
                println!("Opcao invalida, digite os numeros das opcoes");
            }
        }
    }

}
