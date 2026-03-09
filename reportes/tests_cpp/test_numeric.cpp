#include <numeric>
#include <vector>
#include <iostream>

int main() {
    std::vector<int> v = {1, 2, 3, 4, 5};
    int sum = std::accumulate(v.begin(), v.end(), 0);
    int prod = std::accumulate(v.begin(), v.end(), 1, std::multiplies<int>());
    std::vector<int> partial(5);
    std::partial_sum(v.begin(), v.end(), partial.begin());
    std::vector<int> diff(5);
    std::adjacent_difference(v.begin(), v.end(), diff.begin());
    int inner = std::inner_product(v.begin(), v.end(), v.begin(), 0);
    std::vector<int> iota_v(10);
    std::iota(iota_v.begin(), iota_v.end(), 1);
    int gcd_val = std::gcd(12, 8);
    int lcm_val = std::lcm(12, 8);
    std::cout << "sum=" << sum << " gcd=" << gcd_val << std::endl;
    return 0;
}
