// Cryptarithmetic solver via backtracking over distinct letters. O(10!/(10-k)!) worst, pruned.
#include <bits/stdc++.h>
using namespace std;

string W1 = "SEND", W2 = "MORE", W3 = "MONEY";
vector<char> letters;        // first-appearance order
map<char, int> assign;       // letter -> digit
bool usedDigit[10] = {false};
set<char> leading;

long long val(const string& w) {
    long long v = 0;
    for (char c : w) v = v * 10 + assign[c];
    return v;
}

bool solve(int idx) {
    if (idx == (int)letters.size())
        return val(W1) + val(W2) == val(W3);
    char c = letters[idx];
    for (int d = 0; d <= 9; ++d) {
        if (usedDigit[d]) continue;
        if (d == 0 && leading.count(c)) continue;
        usedDigit[d] = true; assign[c] = d;
        if (solve(idx + 1)) return true;
        usedDigit[d] = false;
    }
    return false;
}

int main() {
    leading = {W1[0], W2[0], W3[0]};
    set<char> seen;
    for (const string& w : {W1, W2, W3})
        for (char c : w)
            if (!seen.count(c)) { seen.insert(c); letters.push_back(c); }
    solve(0);
    cout << "{";
    for (size_t i = 0; i < letters.size(); ++i)
        cout << "'" << letters[i] << "': " << assign[letters[i]] << (i + 1 < letters.size() ? ", " : "");
    cout << "}" << endl;
    return 0;
}
