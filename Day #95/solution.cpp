// Day 95: Next lexicographic permutation in place. Find rightmost ascent, swap
// with next-larger suffix element, reverse suffix. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

void nextPermutation(vector<int>& a) {
    int n = a.size(), i = n - 2;
    while (i >= 0 && a[i] >= a[i + 1]) i--;
    if (i >= 0) {
        int j = n - 1;
        while (a[j] <= a[i]) j--;
        swap(a[i], a[j]);
    }
    reverse(a.begin() + i + 1, a.end());
}

int main() {
    vector<vector<int>> tests = {{1, 2, 3}, {1, 3, 2}, {3, 2, 1}};
    for (auto a : tests) {
        nextPermutation(a);
        cout << "[";
        for (size_t i = 0; i < a.size(); i++) cout << a[i] << (i + 1 < a.size() ? ", " : "");
        cout << "]\n";
    }
    // [1, 3, 2] / [2, 1, 3] / [1, 2, 3]
    return 0;
}
