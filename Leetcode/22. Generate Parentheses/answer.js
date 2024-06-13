/**
 * @param {number} n
 * @return {string[]}
 */

var generateParenthesis = function (n) {
  const result = [];
  const recursionGenerator = function (leftCount, rightCount, str, targetLength) {
    if (targetLength == str.length) {
      result.push(str);
      return;
    }

    if (leftCount > 0) {
      recursionGenerator(leftCount - 1, rightCount + 1, str + "(", targetLength);
    }
    if (rightCount > 0) {
      recursionGenerator(leftCount, rightCount - 1, str + ")", targetLength);
    }
  }
  recursionGenerator(n, 0, "", 2 * n);
  return result
};
