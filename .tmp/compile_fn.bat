@echo off
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat" >nul
cd /d D:\zap
cl /nologo /W0 .tmp\fn_emit.c /Fe:.tmp\fn_emit.exe
exit /b %errorlevel%
