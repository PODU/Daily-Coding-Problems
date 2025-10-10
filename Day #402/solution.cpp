// Strobogrammatic numbers of N digits: recursive build outside-in, skip leading 0 pair.
// Time O(5^(N/2)) results, Space O(N) recursion depth.
#include <iostream>
#include <vector>
#include <string>
using namespace std;

vector<string> build(int n, int total) {
    if (n == 0) return {""};
    if (n == 1) return {"0", "1", "8"};
    vector<pair<char,char>> pairs = {{'0','0'},{'1','1'},{'6','9'},{'8','8'},{'9','6'}};
    vector<string> inner = build(n - 2, total);
    vector<string> res;
    for (auto& s : inner) {
        for (auto& p : pairs) {
            if (n == total && p.first == '0') continue; // no leading zero
            res.push_back(string(1, p.first) + s + string(1, p.second));
        }
    }
    return res;
}

int main() {
    vector<string> res = build(2, 2);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << "\"" << res[i] << "\"";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
