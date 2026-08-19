@echo off
setlocal
cd /d "%~dp0"
cargo build --release --manifest-path native\Cargo.toml
if errorlevel 1 exit /b 1
if not exist "bin" mkdir "bin"
copy /Y "native\target\release\zap.exe" "bin\zap.exe" >nul
echo Built standalone binary: %~dp0bin\zap.exe
echo Run: bin\zap.exe native_hello.zp
