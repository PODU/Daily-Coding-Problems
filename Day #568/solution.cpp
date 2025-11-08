// Square a sorted array and return sorted. Two pointers from both ends pick larger
// absolute value into the back of the result. O(N) time, O(N) space.
#include <bits/stdc++.h>
using namespace std;

vector<long long> sortedSquares(const vector<int>& a) {
    int n = a.size(), l = 0, r = n - 1;
    vector<long long> res(n);
    for (int k = n - 1; k >= 0; --k) {
        long long lv = (long long)a[l] * a[l], rv = (long long)a[r] * a[r];
        if (lv > rv) { res[k] = lv; ++l; }
        else { res[k] = rv; --r; }
    }
    return res;
}

int main() {
    auto res = sortedSquares({-9, -2, 0, 2, 3});
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n"; // [0, 4, 4, 9, 81]
    return 0;
}
