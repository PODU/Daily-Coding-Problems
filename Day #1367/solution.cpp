// Cryptarithmetic solver via backtracking over letter->digit assignments.
// Time O(10!/(10-L)!) worst with pruning, Space O(L). L = #distinct letters.
#include <bits/stdc++.h>
using namespace std;

string A, B, C;
vector<char> letters;
set<char> leading;
map<char,int> assign;
bool used[10];

long long val(const string& w) {
    long long v = 0;
    for (char ch : w) v = v * 10 + assign[ch];
    return v;
}

bool bt(int i) {
    if (i == (int)letters.size()) return val(A) + val(B) == val(C);
    char ch = letters[i];
    for (int d = 0; d < 10; d++) {
        if (used[d]) continue;
        if (d == 0 && leading.count(ch)) continue;
        used[d] = true; assign[ch] = d;
        if (bt(i + 1)) return true;
        used[d] = false;
    }
    return false;
}

int main() {
    A = "SEND"; B = "MORE"; C = "MONEY";
    set<char> seen;
    for (string* w : {&A, &B, &C}) {
        leading.insert((*w)[0]);
        for (char ch : *w) if (!seen.count(ch)) { seen.insert(ch); letters.push_back(ch); }
    }
    if (bt(0)) {
        cout << "{";
        for (size_t i = 0; i < letters.size(); i++) {
            cout << "'" << letters[i] << "': " << assign[letters[i]];
            if (i + 1 < letters.size()) cout << ", ";
        }
        cout << "}\n";
    } else cout << "No solution\n";
    return 0;
}
