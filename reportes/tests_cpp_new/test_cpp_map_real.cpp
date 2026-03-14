#include <header_main.h>
#include <map>
#include <string>
#include <vector>

int main() {
    // std::map — ordered by key
    std::map<std::string, int> m;
    m["banana"] = 3;
    m["apple"] = 1;
    m["cherry"] = 2;

    // iterate ordered
    for (auto& [k, v] : m) {
        printf("%s:%d ", k.c_str(), v);
    }
    printf("\n");

    // find/count
    printf("count apple: %d\n", m.count("apple"));
    printf("count grape: %d\n", m.count("grape"));
    printf("size: %d\n", m.size());

    // erase
    m.erase("cherry");
    printf("after erase size: %d\n", m.size());

    // unordered_map — frequency counter
    std::unordered_map<std::string, int> freq;
    std::vector<std::string> words = {"hi", "bye", "hi", "hi", "bye"};
    for (auto& w : words) freq[w]++;
    printf("hi: %d\n", freq["hi"]);
    printf("bye: %d\n", freq["bye"]);

    return 0;
}
