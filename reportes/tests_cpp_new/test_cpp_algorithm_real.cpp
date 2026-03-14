#include <header_main.h>
#include <vector>
#include <algorithm>
#include <numeric>

int main() {
    std::vector<int> v = {5, 3, 8, 1, 9, 2, 7, 4, 6};

    // sort
    std::sort(v.begin(), v.end());
    printf("sorted: ");
    for (auto x : v) printf("%d ", x);
    printf("\n");

    // accumulate
    int sum = std::accumulate(v.begin(), v.end(), 0);
    printf("sum: %d\n", sum);

    // find
    auto it = std::find(v.begin(), v.end(), 7);
    printf("found 7: %d\n", it != v.end());

    // count_if (evens)
    int evens = std::count_if(v.begin(), v.end(), [](int x) { return x % 2 == 0; });
    printf("evens: %d\n", evens);

    // reverse
    std::reverse(v.begin(), v.end());
    printf("reversed[0]: %d\n", v[0]);

    // min/max
    std::vector<int> w = {10, 3, 7, 1, 8};
    auto mn = std::min_element(w.begin(), w.end());
    auto mx = std::max_element(w.begin(), w.end());
    printf("min: %d max: %d\n", *mn, *mx);

    // binary search (on sorted data)
    std::vector<int> sorted = {1, 2, 3, 4, 5, 6, 7, 8, 9};
    printf("bsearch 5: %d\n", std::binary_search(sorted.begin(), sorted.end(), 5));
    printf("bsearch 99: %d\n", std::binary_search(sorted.begin(), sorted.end(), 99));

    return 0;
}
