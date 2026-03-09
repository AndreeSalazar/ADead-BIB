#include <regex>
#include <random>
#include <iostream>

int main() {
    // regex
    std::regex pattern("\\d+");
    std::string text = "abc 123 def 456";
    std::smatch match;
    bool found = std::regex_search(text, match, pattern);
    bool full = std::regex_match("12345", pattern);
    std::string replaced = std::regex_replace(text, pattern, "NUM");

    // random
    std::mt19937 gen(42);
    std::uniform_int_distribution<int> dist(1, 100);
    int r1 = dist(gen);
    
    std::uniform_real_distribution<double> rdist(0.0, 1.0);
    double r2 = rdist(gen);

    std::normal_distribution<double> ndist(0.0, 1.0);
    double r3 = ndist(gen);

    std::bernoulli_distribution bdist(0.5);
    bool r4 = bdist(gen);

    std::default_random_engine engine;
    
    std::cout << "regex found=" << found << " random=" << r1 << std::endl;
    return 0;
}
