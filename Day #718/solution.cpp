// Day 718: Gray code for n bits via formula g(i) = i ^ (i >> 1). Produces 2^n
// values where consecutive (and wrap-around) differ by one bit. Time O(2^n).
#include <bits/stdc++.h>
using namespace std;

vector<string> grayCode(int n) {
    vector<string> res;
    for (int i = 0; i < (1 << n); ++i) {
        int g = i ^ (i >> 1);
        string s;
        for (int b = n - 1; b >= 0; --b) s += ((g >> b) & 1) ? '1' : '0';
        res.push_back(s);
    }
    return res;
}

int main() {
    auto codes = grayCode(2);
    cout << "[";
    for (size_t i = 0; i < codes.size(); ++i) cout << codes[i] << (i + 1 < codes.size() ? ", " : "");
    cout << "]" << endl;
    return 0;
}
