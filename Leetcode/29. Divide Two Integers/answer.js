/**
 * @param {number} dividend
 * @param {number} divisor
 * @return {number}
 */
var divide = function (dividend, divisor) {
  let dividendNeg = 1;
  let divisorNeg = 1;
  if (dividend < 0) {
    dividendNeg = -1;
    dividend = -dividend;
  }
  if (divisor < 0) {
    divisorNeg = -1;
    divisor = -divisor;
  }
  const dividendStr = dividend.toString();
  let result = 0;
  let remain = 0;
  for (const i in dividendStr) {
    const currentNumber = parseInt(dividendStr[i]);
    let currentResult = 0;
    remain = remain * 10 + currentNumber;
    while (remain >= divisor) {
      remain -= divisor;
      currentResult++;
    }
    result = result * 10 + currentResult;
  }
  result = result * divisorNeg * dividendNeg
  if (result > 2147483647)
    return 2147483647;
  if (result < -2147483648)
    return -2147483648
  return result;
};


console.log(divide(10, 3))
console.log(divide(7, 3))
console.log(divide(7, -3))
console.log(divide(-2147483648, -1))
