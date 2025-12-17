// Day 756: Recover digits from an anagram of their English spellings.
// Use unique marker letters (z,w,u,x,g) then deduce the rest. Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

string recoverDigits(const string& s) {
    int cnt[26] = {0};
    for (char c : s) cnt[c - 'a']++;
    int d[10] = {0};
    d[0] = cnt['z'-'a'];
    d[2] = cnt['w'-'a'];
    d[4] = cnt['u'-'a'];
    d[6] = cnt['x'-'a'];
    d[8] = cnt['g'-'a'];
    d[1] = cnt['o'-'a'] - d[0] - d[2] - d[4];
    d[3] = cnt['h'-'a'] - d[8];
    d[5] = cnt['f'-'a'] - d[4];
    d[7] = cnt['s'-'a'] - d[6];
    d[9] = cnt['i'-'a'] - d[5] - d[6] - d[8];

    string out;
    for (int i = 0; i < 10; ++i)
        out.append(d[i], char('0' + i));
    return out;
}

int main() {
    cout << recoverDigits("niesevehrtfeev") << "\n";  // 357
    return 0;
}
