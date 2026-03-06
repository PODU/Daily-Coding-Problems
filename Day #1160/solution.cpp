// Recover digits from scrambled English spellings using unique identifying letters.
// Time: O(L), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

string recover(const string& s) {
    int c[26] = {0};
    for (char ch : s) c[ch - 'a']++;
    int cnt[10] = {0};
    cnt[0] = c['z' - 'a'];
    cnt[2] = c['w' - 'a'];
    cnt[4] = c['u' - 'a'];
    cnt[6] = c['x' - 'a'];
    cnt[8] = c['g' - 'a'];
    cnt[3] = c['h' - 'a'] - cnt[8];
    cnt[5] = c['f' - 'a'] - cnt[4];
    cnt[7] = c['s' - 'a'] - cnt[6];
    cnt[1] = c['o' - 'a'] - cnt[0] - cnt[2] - cnt[4];
    cnt[9] = c['i' - 'a'] - cnt[5] - cnt[6] - cnt[8];
    string out;
    for (int d = 0; d <= 9; ++d)
        out.append(cnt[d], char('0' + d));
    return out;
}

int main() {
    cout << recover("niesevehrtfeev") << "\n";
    return 0;
}
