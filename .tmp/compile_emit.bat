@echo off
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat" >nul
cd /d D:\zap
cl /nologo /W0 .tmp\zap_emit_builtin.c /Fe:.tmp\zap_emit_builtin.exe
exit /b %errorlevel%
