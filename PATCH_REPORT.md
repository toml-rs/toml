# Patch Report: Fix UTF-8 multi-byte span highlight length (Issue #1120)

## Обзор проблемы
В репозитории `toml-rs/toml` при отображении синтаксических ошибок парсера подчеркивание места ошибки (каретки `^`) рассчитывалось некорректно для строк, содержащих многобайтовые символы UTF-8 (например, `åäö`, `μ`, `٠` и др.). 

### Пример ошибки до патча:
```toml
1 | num = 'åäö'
  |       ^^^^^^^^
```
Слово `'åäö'` состоит из 5 символов, но занимает 8 байт в кодировке UTF-8. Из-за расчета длины подчеркивания в байтах отрисовывалось 8 кареток вместо 5, что приводило к подчеркиванию лишних пробелов или последующих токенов.

---

## Локализация
Проблема локализована в модулях форматирования ошибок парсинга:
1. `crates/toml/src/de/error.rs` — реализация `core::fmt::Display for Error`
2. `crates/toml_edit/src/error.rs` — реализация `std::fmt::Display for TomlError`

---

## Внесенные изменения

1. **Расчет количества символов подчеркивания**:
   - Было:
     ```rust
     let highlight_len = span.end - span.start;
     let highlight_len = highlight_len.min(content.len().saturating_sub(column));
     ```
   - Стало:
     ```rust
     let highlight_len = input
         .get(span.clone())
         .map(|s| s.chars().count())
         .unwrap_or_else(|| span.end.saturating_sub(span.start));
     let highlight_len = highlight_len.min(content.chars().count().saturating_sub(column));
     ```
   - Срез строки `input.get(span.clone())` возвращает реальный срез `&str` для заданного диапазона байтов, после чего `.chars().count()` вычисляет точное число символов.
   - В случае выходов за границы или срезов не по границам UTF-8 предусмотрен безопасный фолбэк на разность байтов `span.end.saturating_sub(span.start)`.
   - Ограничение по длине строки переведено с байтового `content.len()` на `content.chars().count()`, устраняя смешивание байтов и символов с позицией `column`.

2. **Регрессионные тесты**:
   - В `crates/toml/tests/serde/de_errors.rs` и `crates/toml_edit/tests/serde/de_errors.rs` добавлен воспроизводящий тест с `num = 'åäö'`:
     ```rust
     #[derive(Deserialize, Debug)]
     #[allow(dead_code)]
     struct Issue1120 {
         num: u32,
     }

     bad::<Issue1120>(
         "num = 'åäö'",
         str![[r#"
     TOML parse error at line 1, column 7
       |
     1 | num = 'åäö'
       |       ^^^^^
     invalid type: string "åäö", expected u32

     "#]],
     );
     ```

3. **Обновление комплаенс-снапшотов**:
   - Актуализированы снапшоты в `crates/toml/tests/snapshots/` и `crates/toml_edit/tests/snapshots/`, ранее фиксировавшие завышенную ширину спана для многобайтовых символов UTF-8.

---

## Верификация
- `cargo clippy --workspace --all-targets` завершился с кодом 0 (без ошибок и предупреждений).
- `cargo test --workspace` полностью успешен (все unit-тесты, compliance-тесты, serde-тесты и doctests пройдены).
