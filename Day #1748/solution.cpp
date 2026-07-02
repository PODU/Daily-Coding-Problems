// Day 1748: Recover digits from an anagram of concatenated number words (zero-nine).
// Approach: unique-letter signatures (z,w,u,x,g first; then derive odd digits). O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

string recover(const string& s) {
    int c[26] = {0};
    for (char ch : s) c[ch - 'a']++;
    int cnt[10] = {0};
    cnt[0] = c['z' - 'a'];                       // zero
    cnt[2] = c['w' - 'a'];                       // two
    cnt[4] = c['u' - 'a'];                       // four
    cnt[6] = c['x' - 'a'];                       // six
    cnt[8] = c['g' - 'a'];                       // eight
    cnt[3] = c['h' - 'a'] - cnt[8];              // three (h also in eight)
    cnt[5] = c['f' - 'a'] - cnt[4];              // five  (f also in four)
    cnt[7] = c['s' - 'a'] - cnt[6];              // seven (s also in six)
    cnt[1] = c['o' - 'a'] - cnt[0] - cnt[2] - cnt[4]; // one (o in zero,two,four)
    cnt[9] = c['i' - 'a'] - cnt[5] - cnt[6] - cnt[8]; // nine (i in five,six,eight)

    string res;
    for (int d = 0; d <= 9; ++d)
        for (int k = 0; k < cnt[d]; ++k) res += char('0' + d);
    return res;
}

int main() {
    cout << recover("niesevehrtfeev") << "\n"; // 357
    return 0;
}
