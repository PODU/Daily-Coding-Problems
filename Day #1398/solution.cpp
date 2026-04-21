// Single scan: count drops; on a drop, greedily fix by lowering nums[i] or
// raising nums[i+1] depending on nums[i-1]. >1 drop => false. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool checkPossibility(vector<int> a) {
    int cnt = 0;
    for (int i = 0; i + 1 < (int)a.size(); i++) {
        if (a[i] > a[i + 1]) {
            if (++cnt > 1) return false;
            if (i == 0 || a[i - 1] <= a[i + 1]) a[i] = a[i + 1];
            else a[i + 1] = a[i];
        }
    }
    return true;
}

int main() {
    cout << (checkPossibility({10, 5, 7}) ? "true" : "false") << "\n";
    cout << (checkPossibility({10, 5, 1}) ? "true" : "false") << "\n";
    return 0;
}
