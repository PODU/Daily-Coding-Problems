# Day 1648: Simplify Unix absolute path: split on '/', push names on a stack, skip ''/'.', pop on '..'. Time O(n), Space O(n).
# Build "/a/b" from the stack; if input ended with '/' and result isn't root, append a trailing '/'.

def simplify_path(path):
    stack = []
    for token in path.split("/"):
        if token == "" or token == ".":
            continue
        if token == "..":
            if stack:
                stack.pop()
        else:
            stack.append(token)
    result = "/" + "/".join(stack)
    ends_with_slash = path.endswith("/")
    if ends_with_slash and result != "/":
        result += "/"
    return result

if __name__ == "__main__":
    print(simplify_path("/usr/bin/../bin/./scripts/../"))
