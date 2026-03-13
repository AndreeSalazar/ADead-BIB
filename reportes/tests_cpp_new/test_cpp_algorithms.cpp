#include <header_main.h>
#include <iostream>
#include <vector>
#include <algorithm>
#include <numeric>
#include <functional>

int main() {
    std::vector<int> v = {5, 3, 1, 4, 2};
    std::sort(v.begin(), v.end());
    printf("sorted: ");
    for (auto& x : v) printf("%d ", x);
    printf("\n");
    auto it = std::find(v.begin(), v.end(), 3);
    printf("find(3): %d\n", *it);
    int sum = std::accumulate(v.begin(), v.end(), 0);
    printf("sum=%d\n", sum);
    std::vector<int> doubled(v.size());
    std::transform(v.begin(), v.end(), doubled.begin(), [](int x) { return x * 2; });
    printf("doubled: ");
    for (auto& x : doubled) printf("%d ", x);
    printf("\n");
    int evens = std::count_if(v.begin(), v.end(), [](int x) { return x % 2 == 0; });
    printf("evens=%d\n", evens);
    std::for_each(v.begin(), v.end(), [](int x) { printf("[%d]", x); });
    printf("\n");
    auto mn = std::min_element(v.begin(), v.end());
    auto mx = std::max_element(v.begin(), v.end());
    printf("min=%d max=%d\n", *mn, *mx);
    bool found = std::binary_search(v.begin(), v.end(), 4);
    printf("binary_search(4)=%d\n", found);
    std::vector<int> v2 = {1, 2, 3, 4, 5, 6};
    int product = std::accumulate(v2.begin(), v2.end(), 1, [](int a, int b) { return a * b; });
    printf("product=%d\n", product);
    return 0;
}
