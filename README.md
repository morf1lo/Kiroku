## Kiroku - is a clipboard manager that stores history of copied text and images

# How to run?

## Download release
Go to `Releases` and download `kiroku.zip` archive. Extract it and run `kiroku.exe` file

## Development

You need [Rust](https://rust-lang.org) and [Node.js](https://nodejs.org) installed

```bash
git clone https://github.com/morf1lo/Kiroku.git
cd Kiroku
npm i
```
#### Development
npm:
```
npm run tauri dev
```
cargo:
```
cargo tauri dev
```
#### Build
npm:
```
npm run tauri build
```
cargo:
```
cargo tauri build
```

it will appear in `src-tauri/target/release`
