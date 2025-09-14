// Day 273: Fixed point (arr[i]==i) in sorted distinct array via binary search.
// Time O(log N), Space O(1). Returns index or -1 (False).
#include <bits/stdc++.h>
using namespace std;

int fixedPoint(const vector<int>& a) {
    int lo = 0, hi = (int)a.size() - 1;
    while (lo <= hi) {
        int mid = lo + (hi - lo) / 2;
        if (a[mid] == mid) return mid;
        else if (a[mid] < mid) lo = mid + 1;
        else hi = mid - 1;
    }
    return -1; // False
}

void show(const vector<int>& a) {
    int r = fixedPoint(a);
    if (r == -1) cout << "False\n";
    else cout << r << "\n";
}

int main() {
    show({-6, 0, 2, 40}); // 2
    show({1, 5, 7, 8});   // False
    return 0;
}
