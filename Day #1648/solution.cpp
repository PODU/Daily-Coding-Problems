// Simplify Unix absolute path: split on '/', push names on a stack, skip ''/'.', pop on '..'. Time O(n), Space O(n).
// Build "/a/b" from the stack; if input ended with '/' and result isn't root, append a trailing '/'.
#include <iostream>
#include <string>
#include <vector>
using namespace std;

string simplifyPath(const string& path) {
    vector<string> stack;
    string token;
    vector<string> tokens;
    for (char ch : path) {
        if (ch == '/') {
            tokens.push_back(token);
            token.clear();
        } else {
            token.push_back(ch);
        }
    }
    tokens.push_back(token);
    for (const string& t : tokens) {
        if (t.empty() || t == ".") continue;
        if (t == "..") {
            if (!stack.empty()) stack.pop_back();
        } else {
            stack.push_back(t);
        }
    }
    string result = "/";
    for (size_t i = 0; i < stack.size(); i++) {
        result += stack[i];
        if (i + 1 < stack.size()) result += "/";
    }
    bool endsWithSlash = !path.empty() && path.back() == '/';
    if (endsWithSlash && result != "/") result += "/";
    return result;
}

int main() {
    cout << simplifyPath("/usr/bin/../bin/./scripts/../") << endl;
    return 0;
}
