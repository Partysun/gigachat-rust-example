# Работа с GigaChat на Rust

## TLDR

Как работать с Gigachat на Rust? На Rust нету gigachain или langchain. Но что-то то есть. Этот репо демонстрирует, как делать запросы к Gigachat API без своей реализации выпуска токена и обработки ошибок.

## Простые шаги

```bash
git clone https://github.com/Partysun/gigachat-rust-example.git
```

```bash
cd gigachat-rust-example
```

Установите ваш ключ в main.rs
Этот шаг в этом примере обязательный.

```bash
cargo run
```

---

P.S.

Чтобы установить ключ, как environment. Просто удалите из билдера шаг с установкой ключа и установите env.

```bash
# On macOS/Linux
export GIGACHAT_AUTH_TOKEN='YTAxNj...'
```

```powershell
# On Windows Powershell
$Env:GIGACHAT_AUTH_TOKEN='YTAxNj...'
```
