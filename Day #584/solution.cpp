// Greedy rearrange: at each step pick highest remaining count != prev char, tie by smallest char.
// Feasible iff maxCount <= (n+1)/2. Time O(n*26), Space O(26).
#include <iostream>
#include <string>
#include <array>
using namespace std;

string rearrange(const string& s) {
    array<int,26> cnt{}; cnt.fill(0);
    for (char c : s) cnt[c - 'a']++;
    int n = (int)s.size();
    string res;
    int prev = -1;
    for (int k = 0; k < n; k++) {
        int best = -1;
        for (int i = 0; i < 26; i++) {
            if (i == prev) continue;
            if (cnt[i] <= 0) continue;
            if (best == -1 || cnt[i] > cnt[best]) best = i; // tie keeps smallest i
        }
        if (best == -1) return ""; // impossible
        res += char('a' + best);
        cnt[best]--;
        prev = best;
    }
    return res;
}

int main() {
    string a = rearrange("aaabbc");
    cout << (a.empty() ? "None" : a) << "\n";
    string b = rearrange("aaab");
    cout << (b.empty() ? "None" : b) << "\n";
    return 0;
}
