# rust_parser

Библиотека (crate) для парсинга/сериализации/десериализации финансовых данных в несколько форматов и отдельные исполняемые cli приложения (comparer, converter), использующие данную библиотеку. 
Поддерживаемые форматы: 
1. csv - Таблица банковских операций
2. txt - Текстовый формат описания списка операций
3. bin - Бинарное предоставление списка операций

# Пример запуска
1. Тесты - "cargo test"
2. Запуск comparer - "cargo run --bin comparer -- --file1 records_example.bin --format1 bin --file2 records_example.txt --format2 txt"
3. Запуск converter - "cargo run --bin converter -- --input records_example.bin --input-format bin --output-format txt"