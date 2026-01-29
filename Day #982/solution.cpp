// Strobogrammatic numbers of N digits: build recursively from outside in using rotatable
// pairs {0-0,1-1,8-8,6-9,9-6}; skip leading 0 for N>1. Time O(5^(N/2)), Space O(N).
#include <bits/stdc++.h>
using namespace std;

vector<string> build(int n, int total) {
    if (n == 0) return {""};
    if (n == 1) return {"0", "1", "8"};
    vector<string> inner = build(n - 2, total);
    static const vector<pair<char,char>> pairs = {{'0','0'},{'1','1'},{'8','8'},{'6','9'},{'9','6'}};
    vector<string> res;
    for (const string& s : inner)
        for (auto& p : pairs) {
            if (p.first == '0' && n == total) continue; // no leading zero
            res.push_back(string(1, p.first) + s + p.second);
        }
    return res;
}

vector<string> strobogrammatic(int n) { return build(n, n); }

int main() {
    cout << "N=2: [";
    auto a = strobogrammatic(2);
    for (size_t i = 0; i < a.size(); i++) cout << a[i] << (i + 1 < a.size() ? ", " : "");
    cout << "]" << endl;
    cout << "N=1: [";
    auto b = strobogrammatic(1);
    for (size_t i = 0; i < b.size(); i++) cout << b[i] << (i + 1 < b.size() ? ", " : "");
    cout << "]" << endl;
    return 0;
}
