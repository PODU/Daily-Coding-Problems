// Day 708: Fixed point (a[i]==i) in a sorted distinct array via binary search.
// Since values are distinct integers, a[i]-i is monotonic. Time O(log n).
#include <bits/stdc++.h>
using namespace std;

// returns index (>=0) or -1 if none
int fixedPoint(const vector<int>& a) {
    int lo = 0, hi = (int)a.size() - 1;
    while (lo <= hi) {
        int mid = (lo + hi) / 2;
        if (a[mid] == mid) return mid;
        else if (a[mid] < mid) lo = mid + 1;
        else hi = mid - 1;
    }
    return -1;
}

void report(const vector<int>& a) {
    int r = fixedPoint(a);
    if (r >= 0) cout << r << endl;
    else cout << "False" << endl;
}

int main() {
    report({-6, 0, 2, 40});
    report({1, 5, 7, 8});
    return 0;
}
