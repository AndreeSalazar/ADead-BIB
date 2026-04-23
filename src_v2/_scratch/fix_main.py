import re

path = r"C:\Users\andre\OneDrive\Documentos\ADead-BIB\src_v2\bin\adeb-main\src\main.rs"

with open(path, "r", encoding="utf-8") as f:
    lines = f.readlines()

new_lines = []
skip = False
for i, line in enumerate(lines):
    # Imports
    if line.startswith("use crate::driver::cpp_driver;"): continue
    if line.startswith("use crate::driver::cuda_driver;"): continue
    if line.startswith("use crate::driver::js_driver;"): continue

    # CLI description
    if "cxx  <file.cpp>" in line: continue
    if "cuda <file.cu>" in line: continue
    if "js   <file.js>" in line: continue
    if "C++ (complete), CUDA (preview), JS (preview)" in line:
        line = line.replace("C++ (complete), CUDA (preview), JS (preview)", "")
    
    # Commands
    if '"cxx" | "c++" | "cpp" => {' in line: skip = True
    if '"cuda" | "cu" => {' in line: skip = True
    if '"js" | "javascript" => {' in line: skip = True
    
    # End of match arms
    if skip and line.strip() == "}":
        skip = False
        continue
    
    if skip: continue

    # Language enum
    if line.strip() == "Cpp,": continue
    if line.strip() == "Cuda,": continue
    if line.strip() == "Js,": continue

    # detect_language
    if "Language::Cpp" in line and "detect_language" in ''.join(lines[i-10:i]): continue
    if "Language::Cuda" in line and "detect_language" in ''.join(lines[i-10:i]): continue
    if "Language::Js" in line and "detect_language" in ''.join(lines[i-10:i]): continue

    # compile_by_language
    if "Language::Cpp => {" in line: skip = True
    if "Language::Cuda => {" in line: skip = True
    if "Language::Js => {" in line: skip = True

    # help output
    if "cxx " in line and "Compile C++" in line: continue
    if "cuda" in line and "Compile CUDA" in line: continue
    if "js  " in line and "Compile JavaScript" in line: continue
    if "C++ is always strict" in line: continue
    if "Compile + run C++" in line: continue
    if "C++ step mode" in line: continue
    if "Auto-detect C++" in line: continue

    # Tests
    if "fn parse_request_cxx()" in line: skip = True
    if "fn parse_request_cuda()" in line: skip = True
    if "fn parse_request_js()" in line: skip = True

    if "detect_language" in line and "Language::Cpp" in line: continue
    if "detect_language" in line and "Language::Cuda" in line: continue
    if "detect_language" in line and "Language::Js" in line: continue
    
    new_lines.append(line)

with open(path, "w", encoding="utf-8") as f:
    f.writelines(new_lines)
