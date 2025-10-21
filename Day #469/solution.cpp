// Mastermind: brute force all 6-permutations of digits 0-9 (10P6=151200),
// keep one consistent with every guess score. Time: O(10P6 * G), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int score(const string& secret, const string& guess) {
    int s = 0;
    for (int i = 0; i < 6; i++) if (secret[i] == guess[i]) s++;
    return s;
}

bool consistent(const vector<pair<string,int>>& guesses) {
    string digits = "0123456789";
    // generate all permutations of length 6
    vector<int> idx = {0,1,2,3,4,5,6,7,8,9};
    // use recursion via std::next_permutation over choose+arrange
    // simpler: iterate all permutations of the 10 digits, take first 6
    sort(idx.begin(), idx.end());
    do {
        string secret;
        for (int i = 0; i < 6; i++) secret += char('0' + idx[i]);
        bool ok = true;
        for (auto& g : guesses)
            if (score(secret, g.first) != g.second) { ok = false; break; }
        if (ok) return true;
        // skip permutations that only differ in the last 4 positions
        // (they produce same first-6 secret); advance past them
    } while (next_permutation(idx.begin(), idx.end()));
    return false;
}

int main() {
    vector<pair<string,int>> ex1 = {{"175286",2},{"293416",3},{"654321",0}};
    vector<pair<string,int>> ex2 = {{"123456",4},{"345678",4},{"567890",4}};
    cout << (consistent(ex1) ? "True" : "False") << "\n";
    cout << (consistent(ex2) ? "True" : "False") << "\n";
    return 0;
}
