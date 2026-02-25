// Smallest window containing every distinct character of the string.
// Sliding window with frequency counts; expand then shrink. O(n) time, O(k) space.
#include <bits/stdc++.h>
using namespace std;

int smallestWindow(const string& s) {
    set<char> distinctSet(s.begin(), s.end());
    int need = distinctSet.size();
    unordered_map<char,int> cnt;
    int have = 0, best = INT_MAX, left = 0;
    for (int right = 0; right < (int)s.size(); right++) {
        if (++cnt[s[right]] == 1) have++;
        while (have == need) {
            best = min(best, right - left + 1);
            if (--cnt[s[left]] == 0) have--;
            left++;
        }
    }
    return best;
}

int main() {
    string s = "jiujitsu";
    cout << smallestWindow(s) << endl;
    return 0;
}
