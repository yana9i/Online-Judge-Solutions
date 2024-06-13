/**
 * @param {string} digits
 * @return {string[]}
 */
var letterCombinations = function (digits) {
  if (digits === '')
    return [];
  const phoneLetterMapper = {
    "2": ['a', 'b', 'c'],
    "3": ['d', 'e', 'f'],
    "4": ['g', 'h', 'i'],
    "5": ['j', 'k', 'l'],
    "6": ['m', 'n', 'o'],
    "7": ['p', 'q', 'r', 's'],
    "8": ['t', 'u', 'v'],
    "9": ['w', 'x', 'y', 'z']
  };
  let result = phoneLetterMapper[digits[0]];
  for (let i in digits) {
    if (i == 0)
      continue;
    result = multiArray(result, phoneLetterMapper[digits[i]])
  }
  return result;
};

const multiArray = (a = [], b = []) => {
  const result = [];
  for (const i of a) {
    for (const j of b) {
      result.push(i + j);
    }
  }
  return result;
}

console.log(letterCombinations("22"));