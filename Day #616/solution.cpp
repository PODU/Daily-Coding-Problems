// Cryptarithmetic solver via backtracking over distinct letters with column-sum check.
// Time: O(10!) worst case over distinct letters (<=10), Space: O(#letters).
#include <bits/stdc++.h>
using namespace std;

string w1, w2, w3;
vector<char> letters;
set<char> leading;
map<char,int> assign;
bool usedDigit[10];

bool valueOf(const string& w, long long& out) {
    out = 0;
    for (char c : w) out = out * 10 + assign[c];
    return true;
}

bool solve(int idx) {
    if (idx == (int)letters.size()) {
        long long a, b, c;
        valueOf(w1, a); valueOf(w2, b); valueOf(w3, c);
        return a + b == c;
    }
    char ch = letters[idx];
    for (int d = 0; d <= 9; d++) {
        if (usedDigit[d]) continue;
        if (d == 0 && leading.count(ch)) continue;
        usedDigit[d] = true; assign[ch] = d;
        if (solve(idx + 1)) return true;
        usedDigit[d] = false;
    }
    return false;
}

int main() {
    w1 = "SEND"; w2 = "MORE"; w3 = "MONEY";
    leading = {w1[0], w2[0], w3[0]};
    set<char> uniq(w1.begin(), w1.end());
    uniq.insert(w2.begin(), w2.end());
    uniq.insert(w3.begin(), w3.end());
    letters.assign(uniq.begin(), uniq.end());
    memset(usedDigit, 0, sizeof(usedDigit));

    if (solve(0)) {
        // Print in S,E,N,D,M,O,R,Y first-appearance order.
        string order; set<char> seen;
        for (char c : w1 + w2 + w3) if (!seen.count(c)) { seen.insert(c); order += c; }
        cout << "{";
        for (size_t i = 0; i < order.size(); i++) {
            if (i) cout << ", ";
            cout << "'" << order[i] << "': " << assign[order[i]];
        }
        cout << "}\n";
    } else {
        cout << "No solution\n";
    }
    return 0;
}
