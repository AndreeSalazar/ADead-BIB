#include <header_main.h>
#include <vector>
#include <algorithm>

int main() {
    // Basic vector operations
    std::vector<int> v;
    v.push_back(10);
    v.push_back(20);
    v.push_back(30);
    printf("size: %d\n", v.size());
    printf("v[0]: %d\n", v[0]);
    printf("v[1]: %d\n", v[1]);
    printf("v[2]: %d\n", v[2]);

    // Initializer list
    std::vector<int> w = {5, 3, 8, 1, 9, 2, 7, 4, 6};
    printf("w size: %d\n", w.size());

    // Sort
    std::sort(w.begin(), w.end());
    for (auto& x : w) printf("%d ", x);
    printf("\n");

    // Range-for with accumulate
    int sum = 0;
    for (auto& x : v) sum += x;
    printf("sum: %d\n", sum);

    // Empty check
    std::vector<int> empty;
    printf("empty: %d\n", empty.empty());

    // Push many elements (test realloc)
    std::vector<int> big;
    for (int i = 0; i < 50; i++) big.push_back(i * i);
    printf("big size: %d\n", big.size());
    printf("big[49]: %d\n", big[49]);

    return 0;
}
