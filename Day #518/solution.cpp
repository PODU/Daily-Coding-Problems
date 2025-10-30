// 3-sum decision: sort, fix one element, two-pointer scan the rest. O(n^2) time, O(1) extra.
#include <bits/stdc++.h>
using namespace std;

bool threeSum(vector<int> a, int k) {
    sort(a.begin(), a.end());
    int n = a.size();
    for (int i = 0; i < n - 2; i++) {
        int lo = i + 1, hi = n - 1;
        while (lo < hi) {
            long long s = (long long)a[i] + a[lo] + a[hi];
            if (s == k) return true;
            if (s < k) lo++; else hi--;
        }
    }
    return false;
}

int main() {
    cout << (threeSum({20, 303, 3, 4, 25}, 49) ? "true" : "false") << "\n";
    return 0;
}
