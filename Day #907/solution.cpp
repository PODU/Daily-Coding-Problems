// Reflected binary Gray code: value i -> i ^ (i>>1) for i in 0..2^n. O(2^n) time/space.
#include <bits/stdc++.h>
using namespace std;

int main() {
    int n = 2;
    int total = 1 << n;
    cout << "[";
    for (int i = 0; i < total; i++) {
        int g = i ^ (i >> 1);
        string s;
        for (int b = n - 1; b >= 0; b--) s += ((g >> b) & 1) ? '1' : '0';
        cout << s;
        if (i + 1 < total) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
