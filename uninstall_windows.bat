@echo off
setlocal EnableExtensions

set "ZAP_BIN=%USERPROFILE%\.zap\bin"
set "ZAP_EXE=%ZAP_BIN%\zap.exe"

if exist "%ZAP_EXE%" (
  del /F /Q "%ZAP_EXE%" >nul
  if errorlevel 1 (
    echo Zap executable ကို ဖယ်ရှား၍ မရပါ။
    exit /b 1
  )
  echo Zap executable ကို ဖယ်ရှားပြီးပါပြီ။
) else (
  echo Zap executable မတွေ့ပါ။
)

powershell -NoProfile -ExecutionPolicy Bypass -Command "$bin=[Environment]::ExpandEnvironmentVariables('%ZAP_BIN%'); $current=[Environment]::GetEnvironmentVariable('Path','User'); if ($null -ne $current) { $parts=$current -split ';' | Where-Object { $_ -and ($_ -ne $bin) }; [Environment]::SetEnvironmentVariable('Path', ($parts -join ';'), 'User') }"
if errorlevel 1 (
  echo User PATH မှ Zap entry ကို ဖယ်ရှား၍ မရပါ။
  exit /b 1
)

echo Zap uninstall completed. Command Prompt အသစ်ဖွင့်ပြီး PATH ပြောင်းလဲမှုကို အသုံးပြုပါ။
endlocal
