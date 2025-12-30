@echo off
echo Compilando C++ Library...

REM Intentar con MSVC primero
where cl >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo Usando MSVC...
    cl /LD /O2 /EHsc cpp_lib.cpp
    goto :done
)

REM Intentar con MinGW/g++
where g++ >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo Usando g++...
    g++ -shared -O3 -o cpp_lib.dll cpp_lib.cpp
    goto :done
)

REM Intentar con clang
where clang++ >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo Usando clang++...
    clang++ -shared -O3 -o cpp_lib.dll cpp_lib.cpp
    goto :done
)

echo ERROR: No se encontro compilador C++ (cl, g++, o clang++)
echo Instala Visual Studio Build Tools o MinGW
exit /b 1

:done
echo Listo: cpp_lib.dll
