// Custom reduce/fold (left to right). O(n) time, O(1) extra space.
#include <iostream>
#include <vector>
#include <functional>

int reduce(const std::vector<int>& arr,
           const std::function<int(int, int)>& fn, int init) {
    int acc = init;
    for (int x : arr) acc = fn(acc, x);
    return acc;
}

int main() {
    std::vector<int> v = {1, 2, 3, 4};
    auto add = [](int a, int b) { return a + b; };
    auto mul = [](int a, int b) { return a * b; };
    std::cout << reduce(v, add, 0) << "\n"; // 10
    std::cout << reduce(v, mul, 1) << "\n"; // 24 (bonus)
    return 0;
}
