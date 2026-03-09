#include <filesystem>
#include <iostream>

int main() {
    namespace fs = std::filesystem;
    
    fs::path p = "/home/user/file.txt";
    std::string filename = p.filename().string();
    std::string ext = p.extension().string();
    std::string parent = p.parent_path().string();
    std::string stem = p.stem().string();
    bool exists = fs::exists(p);
    bool is_dir = fs::is_directory(p);
    bool is_file = fs::is_regular_file(p);
    
    fs::path joined = p.parent_path() / "other.txt";
    
    fs::create_directories("/tmp/test/deep");
    
    for (auto& entry : fs::directory_iterator("/tmp")) {
        std::cout << entry.path() << std::endl;
    }
    
    uintmax_t sz = fs::file_size("/tmp/test.txt");
    fs::copy("/tmp/a.txt", "/tmp/b.txt");
    fs::rename("/tmp/old.txt", "/tmp/new.txt");
    fs::remove("/tmp/del.txt");
    
    std::cout << "filename=" << filename << std::endl;
    return 0;
}
