# Day 222: Normalize an absolute path (resolve . and ..).
# Approach: split on '/', push names onto a stack, pop on "..", skip "." and "". Time O(n), Space O(n).
def simplify_path(path: str) -> str:
    st = []
    for tok in path.split("/"):
        if tok == "" or tok == ".":
            continue
        if tok == "..":
            if st:
                st.pop()
        else:
            st.append(tok)
    if not st:
        return "/"
    return "/" + "/".join(st) + "/"  # trailing slash (directory form)


if __name__ == "__main__":
    print(f'"{simplify_path("/usr/bin/../bin/./scripts/../")}"')  # "/usr/bin/"
