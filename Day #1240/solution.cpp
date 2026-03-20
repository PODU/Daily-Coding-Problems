// Reflected binary Gray code: g(i) = i ^ (i>>1). Time O(2^n), Space O(2^n).
#include <bits/stdc++.h>
using namespace std;

vector<string> grayCode(int n) {
    vector<string> res;
    for (int i = 0; i < (1 << n); ++i) {
        int g = i ^ (i >> 1);
        string bits;
        for (int b = n - 1; b >= 0; --b) bits += ((g >> b) & 1) ? '1' : '0';
        res.push_back(bits);
    }
    return res;
}

int main() {
    auto codes = grayCode(2);
    cout << "[";
    for (size_t i = 0; i < codes.size(); ++i) {
        cout << codes[i];
        if (i + 1 < codes.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
