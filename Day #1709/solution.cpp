// Non-decreasing with <=1 change: single pass, on violation greedily lower a[i-1] or raise a[i]. O(n).
#include <bits/stdc++.h>
using namespace std;

bool checkPossibility(vector<int> a) {
    int cnt = 0;
    for (size_t i = 1; i < a.size(); ++i) {
        if (a[i - 1] > a[i]) {
            if (++cnt > 1) return false;
            if (i < 2 || a[i - 2] <= a[i]) a[i - 1] = a[i];
            else a[i] = a[i - 1];
        }
    }
    return true;
}

int main() {
    cout << (checkPossibility({10, 5, 7}) ? "true" : "false") << "\n";
    cout << (checkPossibility({10, 5, 1}) ? "true" : "false") << "\n";
    return 0;
}
