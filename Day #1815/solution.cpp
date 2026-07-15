// Gray code via reflect-and-prefix formula g(i) = i XOR (i>>1).
// Time: O(n * 2^n) to format. Space: O(2^n).
#include <bits/stdc++.h>
using namespace std;

vector<string> grayCode(int n) {
    vector<string> res;
    for (int i = 0; i < (1 << n); i++) {
        int g = i ^ (i >> 1);
        string s;
        for (int b = n - 1; b >= 0; b--) s += ((g >> b) & 1) ? '1' : '0';
        res.push_back(s);
    }
    return res;
}

int main() {
    auto codes = grayCode(2);
    cout << "[";
    for (size_t i = 0; i < codes.size(); i++) {
        if (i) cout << ", ";
        cout << codes[i];
    }
    cout << "]\n"; // [00, 01, 11, 10]
    return 0;
}
