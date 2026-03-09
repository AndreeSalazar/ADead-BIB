#include <initializer_list>
#include <iterator>
#include <vector>
#include <iostream>

class MyContainer {
    std::vector<int> data;
public:
    MyContainer(std::initializer_list<int> il) : data(il) {}
    int size() const { return data.size(); }
    int operator[](int i) const { return data[i]; }
};

int main() {
    // initializer_list
    MyContainer mc = {1, 2, 3, 4, 5};
    
    std::initializer_list<int> il = {10, 20, 30};
    for (auto v : il) {
        std::cout << v << " ";
    }
    std::cout << std::endl;

    // iterator utilities
    std::vector<int> v = {1, 2, 3, 4, 5};
    auto it = v.begin();
    std::advance(it, 2);
    int dist = std::distance(v.begin(), it);
    auto next_it = std::next(v.begin(), 3);
    auto prev_it = std::prev(v.end(), 2);
    
    // back_inserter
    std::vector<int> dest;
    std::copy(v.begin(), v.end(), std::back_inserter(dest));
    
    // reverse_iterator
    for (auto rit = v.rbegin(); rit != v.rend(); ++rit) {
        std::cout << *rit << " ";
    }
    std::cout << std::endl;

    std::cout << "container size=" << mc.size() << std::endl;
    return 0;
}
