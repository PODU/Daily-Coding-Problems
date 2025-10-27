// Day 505: Rotate array right by k in-place via three reversals.
// Time O(n), Space O(1).
#include <iostream>
#include <vector>

void reverse(std::vector<int>& a, int lo, int hi) {
    while (lo < hi) std::swap(a[lo++], a[hi--]);
}

void rotateRight(std::vector<int>& a, int k) {
    int n = (int)a.size();
    if (n == 0) return;
    k %= n;
    reverse(a, 0, n - 1);
    reverse(a, 0, k - 1);
    reverse(a, k, n - 1);
}

void printArray(const std::vector<int>& a) {
    std::cout << "[";
    for (size_t i = 0; i < a.size(); ++i) {
        if (i) std::cout << ", ";
        std::cout << a[i];
    }
    std::cout << "]" << std::endl;
}

int main() {
    std::vector<int> a = {1, 2, 3, 4, 5, 6, 7};
    rotateRight(a, 3);
    printArray(a); // [5, 6, 7, 1, 2, 3, 4]
    return 0;
}
