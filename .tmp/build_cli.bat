@echo off
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat" >nul
cl /nologo /O2 /Fe:target\c-backend-ds\cli.exe target\c-backend-ds\cli.c
echo EXITCODE=%ERRORLEVEL%