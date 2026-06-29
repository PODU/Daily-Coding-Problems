// Binary search for fixed point (A[i]==i) in sorted distinct array; A[i]-i non-decreasing.
// Time: O(log n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

string fixedPoint(const vector<int>& a) {
    int lo = 0, hi = (int)a.size() - 1, ans = -1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        if (a[mid] == mid) { ans = mid; hi = mid - 1; }
        else if (a[mid] < mid) lo = mid + 1;
        else hi = mid - 1;
    }
    return ans == -1 ? "False" : to_string(ans);
}

int main() {
    cout << fixedPoint({-6, 0, 2, 40}) << "\n";
    cout << fixedPoint({1, 5, 7, 8}) << "\n";
    return 0;
}
