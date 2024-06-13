/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function (s) {
  const stack = [];
  for (const i of s) {
    if (i == '(' || i == '{' || i == '[') {
      stack.push(i);
    }
    else {
      const lastChar = stack[stack.length - 1];
      if (lastChar == undefined)
        return false;
      if (i == ')' && lastChar == '(')
        stack.pop();
      else if (i == ']' && lastChar == '[')
        stack.pop();
      else if (i == '}' && lastChar == '{')
        stack.pop()
      else
        return false
    }
  }
  return stack.length === 0;
};


console.log(isValid("(])"))