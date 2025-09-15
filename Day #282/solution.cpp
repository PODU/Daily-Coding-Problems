// Day 282: Detect Pythagorean triplet. Square + sort, then two-pointer per c.
// Time O(N^2), Space O(1) extra (in-place squares).
#include <bits/stdc++.h>
using namespace std;

bool hasTriplet(vector<long long> a) {
    for (auto& x : a) x = x * x;
    sort(a.begin(), a.end());
    int n = (int)a.size();
    for (int c = n - 1; c >= 2; c--) {
        int lo = 0, hi = c - 1;
        while (lo < hi) {
            long long s = a[lo] + a[hi];
            if (s == a[c]) return true;
            else if (s < a[c]) lo++;
            else hi--;
        }
    }
    return false;
}

int main() {
    cout << boolalpha;
    cout << hasTriplet({3, 1, 4, 6, 5}) << "\n"; // true (3,4,5)
    cout << hasTriplet({10, 4, 6, 12, 5}) << "\n"; // false
    return 0;
}
