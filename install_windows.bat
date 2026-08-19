@echo off
setlocal EnableExtensions
cd /d "%~dp0"

REM Python မလိုသော Zap native installer.
set "ZAP_SOURCE=%~dp0bin\zap.exe"
if not exist "%ZAP_SOURCE%" (
  if /I "%ZAP_BUILD_FROM_SOURCE%"=="1" (
    where cargo >nul 2>nul
    if errorlevel 1 (
      echo Rust/cargo မေတြ႕ပါ။ Source build အတြက္ Rust toolchain လိုအပ္ပါသည္။
      exit /b 1
    )
    echo Building Zap native runtime from source...
    cargo build --release --manifest-path "%~dp0native\Cargo.toml"
    if errorlevel 1 exit /b 1
    set "ZAP_SOURCE=%~dp0native\target\release\zap.exe"
  ) else (
    echo Prebuilt Zap binary မေတြ႕ပါ။ Official Windows binary release archive ကို download လုပ္ပါ။
    echo Source build လုပ္လိုပါက ZAP_BUILD_FROM_SOURCE=1 သတ္မွတ္ပါ။
    exit /b 1
  )
)

set "ZAP_BIN=%USERPROFILE%\.zap\bin"
if not exist "%ZAP_BIN%" mkdir "%ZAP_BIN%"
copy /Y "%ZAP_SOURCE%" "%ZAP_BIN%\zap.exe" >nul
setx PATH "%PATH%;%ZAP_BIN%" >nul
set "PATH=%PATH%;%ZAP_BIN%"

echo Zap native installed globally: 
call "%ZAP_BIN%\zap.exe" --version
echo Python မလိုပါ။ Command Prompt အသစ္ဖြင့္ၿပီး မည္သည့္ folder မွာမဆို zap file.zp ဟု run လုပ္ႏိုင္ပါသည္။
endlocal
