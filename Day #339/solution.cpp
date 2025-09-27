// Three entries summing to k: sort + fix one + two-pointer.
// O(n^2) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

bool threeSum(vector<int> a, long long k) {
    sort(a.begin(), a.end());
    int n = (int)a.size();
    for (int i = 0; i < n - 2; ++i) {
        int lo = i + 1, hi = n - 1;
        long long target = k - a[i];
        while (lo < hi) {
            long long s = (long long)a[lo] + a[hi];
            if (s == target) return true;
            if (s < target) ++lo; else --hi;
        }
    }
    return false;
}

int main() {
    vector<int> a = {20, 303, 3, 4, 25};
    cout << (threeSum(a, 49) ? "true" : "false") << '\n';
    return 0;
}
