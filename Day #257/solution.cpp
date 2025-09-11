// Day 257: Smallest window that must be sorted to make the whole array sorted.
// Left->right track max to find right bound; right->left track min to find left bound.
// Time: O(n), Space: O(1).
#include <iostream>
#include <vector>
#include <climits>
using namespace std;

pair<int,int> sortWindow(const vector<int>& a) {
    int n = a.size();
    int begin = -1, end = -1;
    int mx = INT_MIN;
    for (int i = 0; i < n; i++) {
        if (a[i] < mx) end = i;
        else mx = a[i];
    }
    int mn = INT_MAX;
    for (int i = n - 1; i >= 0; i--) {
        if (a[i] > mn) begin = i;
        else mn = a[i];
    }
    return {begin, end};
}

int main() {
    vector<int> a = {3, 7, 5, 6, 9};
    auto r = sortWindow(a);
    cout << "(" << r.first << ", " << r.second << ")\n"; // (1, 3)
    return 0;
}
