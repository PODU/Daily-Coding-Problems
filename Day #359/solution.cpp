// Recover digits from scrambled letters: use unique marker letters (z,w,u,x,g) then derive the rest. O(N) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;
string recover(const string& s) {
    int c[26] = {0};
    for (char ch : s) if (ch >= 'a' && ch <= 'z') c[ch - 'a']++;
    int d[10];
    d[0] = c['z' - 'a'];
    d[2] = c['w' - 'a'];
    d[4] = c['u' - 'a'];
    d[6] = c['x' - 'a'];
    d[8] = c['g' - 'a'];
    d[3] = c['h' - 'a'] - d[8];
    d[5] = c['f' - 'a'] - d[4];
    d[7] = c['s' - 'a'] - d[6];
    d[1] = c['o' - 'a'] - d[0] - d[2] - d[4];
    d[9] = c['i' - 'a'] - d[5] - d[6] - d[8];
    string res;
    for (int i = 0; i < 10; i++)
        res.append(d[i], char('0' + i));
    return res;
}
int main() {
    cout << recover("niesevehrtfeev") << "\n";
    return 0;
}
