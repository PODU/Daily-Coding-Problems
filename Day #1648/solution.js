// Simplify Unix absolute path: split on '/', push names on a stack, skip ''/'.', pop on '..'. Time O(n), Space O(n).
// Build "/a/b" from the stack; if input ended with '/' and result isn't root, append a trailing '/'.
'use strict';

function simplifyPath(path) {
    const stack = [];
    for (const token of path.split("/")) {
        if (token === "" || token === ".") continue;
        if (token === "..") {
            if (stack.length > 0) stack.pop();
        } else {
            stack.push(token);
        }
    }
    let result = "/" + stack.join("/");
    const endsWithSlash = path.endsWith("/");
    if (endsWithSlash && result !== "/") result += "/";
    return result;
}

console.log(simplifyPath("/usr/bin/../bin/./scripts/../"));
