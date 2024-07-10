use anilibria_rlib::title::functions;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let queries = vec!["9000", "kimetsu-no-yaiba"];

        for query in queries {
            println!("Запрос для: {}", query);
            match functions::get_title(query).await {
                Ok(title_info) => {
                    println!("Успешно получены данные:");
                    println!("ID: {}", title_info.id);
                    println!("Код: {}", title_info.code);
                    println!("Русское название: {}", title_info.names.ru);
                    println!("Английское название: {}", title_info.names.en);
                    println!("Описание: {}", title_info.description.as_deref().unwrap_or("Нет описания"));
                    println!("Заблокировано: {}", title_info.blocked.map_or("Нет информации".to_string(), |b| b.blocked.to_string()));
                    println!("---------------------------");
                },
                Err(e) => {
                    eprintln!("Ошибка при получении данных:");
                    eprintln!("{:#}", e);
                    println!("---------------------------");
                },
            }
        }
    });
}