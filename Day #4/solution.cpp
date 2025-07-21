// First missing positive: place each value v in slot v-1 via swaps, then scan.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int firstMissingPositive(vector<int>& a) {
    int n = a.size();
    for (int i = 0; i < n; i++) {
        while (a[i] > 0 && a[i] <= n && a[a[i] - 1] != a[i])
            swap(a[i], a[a[i] - 1]);
    }
    for (int i = 0; i < n; i++) if (a[i] != i + 1) return i + 1;
    return n + 1;
}

int main() {
    vector<int> a = {3, 4, -1, 1};
    cout << firstMissingPositive(a) << "\n";
    return 0;
}
