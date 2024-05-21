/**
 * @param {string} s
 * @param {number} numRows
 * @return {string}
 */
var convert = function (s, numRows) {
  if (numRows === 1)
    return s;
  let rowCursor = 0;
  const resultArray = new Array(numRows).fill("");
  let step = 1;
  for (const item in s) {
    if (rowCursor === 0)
      step = 1;
    if (rowCursor === numRows - 1)
      step = -1;

    resultArray[rowCursor] += s[item];

    rowCursor += step;
  }
  return resultArray.join("");
};

console.log(convert("PAYPALISHIRING", 3));
console.log(convert("PAYPALISHIRING", 4));
console.log(convert("A", 2));