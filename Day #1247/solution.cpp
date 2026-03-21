// Squares of a sorted array via two-pointer merge from both ends. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

vector<int> sortedSquares(const vector<int>& a) {
    int n = a.size();
    vector<int> res(n);
    int l = 0, r = n - 1;
    for (int i = n - 1; i >= 0; --i) {
        int ls = a[l] * a[l], rs = a[r] * a[r];
        if (ls > rs) { res[i] = ls; ++l; }
        else { res[i] = rs; --r; }
    }
    return res;
}

int main() {
    vector<int> input = {-9, -2, 0, 2, 3};
    vector<int> res = sortedSquares(input);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << res[i];
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
