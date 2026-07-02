// Day 1754: All strobogrammatic numbers with N digits.
// Build from middle outward placing rotatable pairs; skip leading 0 for outer layer.
// Time O(output size), space O(N) recursion depth.
#include <bits/stdc++.h>
using namespace std;

vector<string> build(int n, int total) {
    if (n == 0) return {""};
    if (n == 1) return {"0", "1", "8"};
    vector<string> inner = build(n - 2, total);
    static const vector<pair<char,char>> pairs = {
        {'0','0'}, {'1','1'}, {'6','9'}, {'8','8'}, {'9','6'}};
    vector<string> res;
    for (const string& s : inner) {
        for (auto& p : pairs) {
            if (p.first == '0' && n == total) continue; // no leading zero
            res.push_back(p.first + s + p.second);
        }
    }
    return res;
}

vector<string> strobogrammatic(int n) { return build(n, n); }

int main() {
    int n = 2;
    vector<string> r = strobogrammatic(n);
    cout << "N=" << n << ": [";
    for (size_t i = 0; i < r.size(); ++i)
        cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n";

    n = 3;
    r = strobogrammatic(n);
    cout << "N=" << n << ": [";
    for (size_t i = 0; i < r.size(); ++i)
        cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
