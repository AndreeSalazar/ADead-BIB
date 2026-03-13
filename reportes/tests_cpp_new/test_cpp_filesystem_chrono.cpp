#include <header_main.h>
#include <iostream>
#include <filesystem>
#include <chrono>
#include <string>

int main() {
    namespace fs = std::filesystem;
    fs::path p = "/home/user/file.txt";
    std::string filename = p.filename().string();
    std::string ext = p.extension().string();
    std::string parent = p.parent_path().string();
    std::string stem = p.stem().string();
    printf("file=%s ext=%s parent=%s stem=%s\n", filename.c_str(), ext.c_str(), parent.c_str(), stem.c_str());
    fs::path joined = p.parent_path() / "other.txt";
    printf("joined=%s\n", joined.string().c_str());
    bool exists = fs::exists(p);
    bool is_dir = fs::is_directory(p);
    bool is_file = fs::is_regular_file(p);
    printf("exists=%d dir=%d file=%d\n", exists, is_dir, is_file);
    auto start = std::chrono::high_resolution_clock::now();
    int sum = 0;
    for (int i = 0; i < 1000; i++) sum += i;
    auto end = std::chrono::high_resolution_clock::now();
    auto ms = std::chrono::duration_cast<std::chrono::microseconds>(end - start);
    printf("sum=%d time=%d us\n", sum, ms.count());
    std::chrono::milliseconds delay(100);
    std::chrono::seconds sec(1);
    printf("chrono OK\n");
    return 0;
}
