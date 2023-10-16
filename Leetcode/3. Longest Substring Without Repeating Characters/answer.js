/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstring = function (s) {
  const charSet = new Set();
  let max = 0;
  for (let i = 0, j = 0; i < s.length; i++) {
    console.log(charSet);
    if (charSet.has(s[i])) {
      while (charSet.has(s[i])) {
        charSet.delete(s[j++]);
      }
      charSet.add(s[i]);
    } else {
      charSet.add(s[i]);
      max = max < charSet.size ? charSet.size : max;
    }
  }
  return max;
};

console.log(lengthOfLongestSubstring(""));
// console.log(lengthOfLongestSubstring("aabaab!bb"));
// console.log(lengthOfLongestSubstring("pwwkew"));
// console.log(lengthOfLongestSubstring("bbbbbbb"));
