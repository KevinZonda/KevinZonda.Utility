@echo off
if not exist "bin" mkdir bin
cd bin

echo "[+] build replacer"
rustc "..\\kevinzonda_utility_rust\\src\\replacer\\main.rs" -o replacer.exe
pause