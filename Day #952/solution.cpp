// Day 952: decode an anagram of spelled-out digit words (zero..nine) -> sorted digits.
// Use unique marker letters to count each digit. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

string decode(const string& s) {
    int c[26] = {0};
    for (char ch : s) c[ch - 'a']++;
    int cnt[10] = {0};
    cnt[0] = c['z' - 'a'];                         // zero
    cnt[2] = c['w' - 'a'];                         // two
    cnt[4] = c['u' - 'a'];                         // four
    cnt[6] = c['x' - 'a'];                         // six
    cnt[8] = c['g' - 'a'];                         // eight
    cnt[3] = c['h' - 'a'] - cnt[8];                // three (h: three,eight)
    cnt[5] = c['f' - 'a'] - cnt[4];                // five  (f: four,five)
    cnt[7] = c['s' - 'a'] - cnt[6];                // seven (s: six,seven)
    cnt[1] = c['o' - 'a'] - cnt[0] - cnt[2] - cnt[4]; // one (o: zero,one,two,four)
    cnt[9] = c['i' - 'a'] - cnt[5] - cnt[6] - cnt[8]; // nine (i: five,six,eight,nine)
    string res;
    for (int d = 0; d <= 9; ++d)
        res.append(cnt[d], char('0' + d));
    return res;
}

int main() {
    cout << decode("niesevehrtfeev") << "\n"; // 357
    return 0;
}
