// Day 1152: Simplify absolute Unix path.
// Stack of components; '.' ignored, '..' pops. Time O(n), Space O(n).
function simplify(path) {
  const st = [];
  for (const part of path.split("/")) {
    if (part === "" || part === ".") continue;
    if (part === "..") st.pop();
    else st.push(part);
  }
  return "/" + st.map((p) => p + "/").join(""); // trailing slash preserved
}

console.log(simplify("/usr/bin/../bin/./scripts/../")); // /usr/bin/
