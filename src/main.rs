// Функция для переворота фразы из нескольких слов
fn reverse_string(input: &str) -> String {
    // Разбиваем входную строку на отдельные слова по пробелам
    let words: Vec<&str> = input.split(' ').collect();

    // Создаем пустую строку для обратной фразы
    let mut reversed_phrase = String::new();

    // Перебираем слова в обратном порядке
    for word in words.iter().rev() {
        // Добавляем текущее слово и пробел после слова
        reversed_phrase.push_str(word);
        reversed_phrase.push(' ');
    }

    // Возвращаем обратную фразу
    reversed_phrase
}

fn main() {
    // Задаем входную строку
    let input_string = "snow dog sun";

    // Вызываем функцию для переворота строки
    let reversed = reverse_string(input_string);

    // Выводим исходную и перевернутую строки
    println!("Исходная строка: {}", input_string);
    println!("Перевернутая строка: {}", reversed);
}