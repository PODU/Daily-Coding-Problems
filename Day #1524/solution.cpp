// First missing positive: cyclic sort (index-as-hash), place each x in [1,n] at index x-1.
// Time O(n), Space O(1) extra (in-place).
#include <bits/stdc++.h>
using namespace std;

int firstMissingPositive(vector<int> a) {
    int n = (int)a.size();
    for (int i = 0; i < n; i++) {
        while (a[i] >= 1 && a[i] <= n && a[a[i] - 1] != a[i])
            swap(a[i], a[a[i] - 1]);
    }
    for (int i = 0; i < n; i++)
        if (a[i] != i + 1) return i + 1;
    return n + 1;
}

int main() {
    cout << firstMissingPositive({3, 4, -1, 1}) << "\n"; // 2
    cout << firstMissingPositive({1, 2, 0}) << "\n";     // 3
    return 0;
}
