// Day 694: First missing positive integer in O(n) time, O(1) space.
// Approach: cyclic sort - place each value v in [1,n] at index v-1, then scan.
// Time O(n), Space O(1) (in-place).
#include <bits/stdc++.h>
using namespace std;

int firstMissingPositive(vector<int>& a) {
    int n = a.size();
    for (int i = 0; i < n; ++i)
        while (a[i] > 0 && a[i] <= n && a[a[i] - 1] != a[i])
            swap(a[i], a[a[i] - 1]);
    for (int i = 0; i < n; ++i)
        if (a[i] != i + 1) return i + 1;
    return n + 1;
}

int main() {
    vector<int> a = {3, 4, -1, 1}, b = {1, 2, 0};
    cout << firstMissingPositive(a) << "\n"; // 2
    cout << firstMissingPositive(b) << "\n"; // 3
    return 0;
}
