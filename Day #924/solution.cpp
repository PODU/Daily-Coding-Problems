// Smallest window to sort: scan left->right tracking max for right bound,
// right->left tracking min for left bound. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

pair<int,int> findUnsortedWindow(const vector<int>& a) {
    int n = a.size();
    int right = -1, runMax = a[0];
    for (int i = 1; i < n; i++) {
        if (a[i] < runMax) right = i;
        else runMax = a[i];
    }
    int left = -1, runMin = a[n-1];
    for (int i = n - 2; i >= 0; i--) {
        if (a[i] > runMin) left = i;
        else runMin = a[i];
    }
    return {left, right};
}

int main() {
    vector<int> arr = {3, 7, 5, 6, 9};
    auto r = findUnsortedWindow(arr);
    cout << "(" << r.first << ", " << r.second << ")\n";
    return 0;
}
