@echo off
setlocal EnableExtensions
cd /d "%~dp0"

REM Zap standalone Windows installer.
REM The release archive already contains bin\zap.exe; no extra runtime is required.
set "ZAP_SOURCE=%~dp0bin\zap.exe"
if not exist "%ZAP_SOURCE%" (
  echo Zap binary မတွေ့ပါ: "%ZAP_SOURCE%"
  echo Official Windows archive ထဲက bin\zap.exe ပါသော package ကို အသုံးပြုပါ။
  exit /b 1
)

set "ZAP_BIN=%USERPROFILE%\.zap\bin"
if not exist "%ZAP_BIN%" mkdir "%ZAP_BIN%"
if errorlevel 1 (
  echo Zap installation directory ဖန်တီး၍ မရပါ: "%ZAP_BIN%"
  exit /b 1
)

copy /Y "%ZAP_SOURCE%" "%ZAP_BIN%\zap.exe" >nul
if errorlevel 1 (
  echo zap.exe ကို copy လုပ်၍ မရပါ။
  exit /b 1
)

REM Persist the user-level PATH without setx's length/truncation limitations.
powershell -NoProfile -ExecutionPolicy Bypass -Command "$bin=[Environment]::ExpandEnvironmentVariables('%ZAP_BIN%'); $current=[Environment]::GetEnvironmentVariable('Path','User'); if ([string]::IsNullOrWhiteSpace($current)) {$current=''}; $parts=$current -split ';' | Where-Object { $_ -and ($_ -ne $bin) }; [Environment]::SetEnvironmentVariable('Path', (($parts + $bin) -join ';'), 'User')"
if errorlevel 1 (
  echo User PATH ကို update လုပ်၍ မရပါ။ Direct path ဖြင့် ဆက်သုံးနိုင်ပါသည်:
  echo "%ZAP_BIN%\zap.exe" --version
) else (
  echo User PATH ကို update လုပ်ပြီးပါပြီ။
)

set "PATH=%ZAP_BIN%;%PATH%"
echo.
echo Zap executable ကို install လုပ်ပြီးပါပြီ။
"%ZAP_BIN%\zap.exe" --version
echo.
echo လက်ရှိ Command Prompt မှာ ချက်ချင်းသုံးရန်:
echo   "%ZAP_BIN%\zap.exe" main.zp
echo.
echo Command Prompt အသစ်ဖွင့်ပြီး မည်သည့် folder မှာမဆို သုံးရန်:
echo   zap main.zp
endlocal
