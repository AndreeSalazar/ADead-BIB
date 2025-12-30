@echo off
echo Compilando C++ Library...
g++ -shared -o cpp_lib.dll -fPIC cpp_lib.cpp
echo Listo: cpp_lib.dll
