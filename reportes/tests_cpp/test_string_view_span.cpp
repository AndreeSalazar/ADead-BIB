#include <string_view>
#include <span>
#include <iostream>

void print_view(std::string_view sv) {
    std::cout << sv << " (len=" << sv.size() << ")" << std::endl;
}

void print_span(std::span<int> sp) {
    for (auto& v : sp) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main() {
    // string_view
    std::string_view sv = "Hello World";
    std::string_view sub = sv.substr(0, 5);
    int pos = sv.find("World");
    bool starts = sv.starts_with("Hello");
    bool ends = sv.ends_with("World");
    char first = sv.front();
    char last = sv.back();
    bool empty = sv.empty();
    const char* data = sv.data();
    
    sv.remove_prefix(6);
    print_view(sv);

    // span (C++20)
    int arr[] = {1, 2, 3, 4, 5};
    std::span<int> sp(arr, 5);
    std::span<int> first3 = sp.first(3);
    std::span<int> last2 = sp.last(2);
    std::span<int> mid = sp.subspan(1, 3);
    int sz = sp.size();
    print_span(sp);

    return 0;
}
