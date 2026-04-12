// Find smallest window to sort: right bound = last i where a[i] < running max; left bound = first j where a[j] > running min from right.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

pair<int,int> findUnsorted(const vector<int>& a) {
    int n = a.size();
    int end = -1, mx = INT_MIN;
    for (int i = 0; i < n; ++i) {
        if (a[i] < mx) end = i;
        else mx = a[i];
    }
    int start = -1, mn = INT_MAX;
    for (int i = n - 1; i >= 0; --i) {
        if (a[i] > mn) start = i;
        else mn = a[i];
    }
    return {start, end};
}

int main() {
    vector<int> a = {3, 7, 5, 6, 9};
    auto r = findUnsorted(a);
    cout << "(" << r.first << ", " << r.second << ")\n";
    return 0;
}
