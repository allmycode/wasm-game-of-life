Учебный проект для освоения сборки Rust в Webassemly.
Создан по руководству [отсюда](https://rustwasm.github.io/docs/book/game-of-life/introduction.html)

# Сборка 
в основной папке
```
wasm-pack build
```
это компиляция rust кода в .wasm и .js

в папке `www`
удалить установленный модуль wasm-game-of-life
```
rm -rf node_modules/wasm-game-of-life/
```
затем снова установить его новую версию
```
npm install
```
и наконец запустить сервер
```
npm run start
```
можно соединить все в одной команде
```
rm -rf node_modules/wasm-game-of-life/; npm install && npm run start
```
