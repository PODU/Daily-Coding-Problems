// Longest absolute file path: split on '\n', track pathLen by depth via stack. Time O(n), Space O(depth).
#include <bits/stdc++.h>
using namespace std;
int lengthLongestPath(const string& input) {
    vector<int> pathLen;
    int maxLen = 0;
    string token;
    stringstream ss(input);
    while (getline(ss, token, '\n')) {
        int depth = 0;
        while (depth < (int)token.size() && token[depth] == '\t') depth++;
        string name = token.substr(depth);
        int curLen = (depth > 0 ? pathLen[depth - 1] + 1 : 0) + (int)name.size();
        if ((int)pathLen.size() == depth) pathLen.push_back(curLen);
        else pathLen[depth] = curLen;
        if (name.find('.') != string::npos)
            maxLen = max(maxLen, curLen);
    }
    return maxLen;
}
int main() {
    string input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
    cout << lengthLongestPath(input) << endl;
    return 0;
}
