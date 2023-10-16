/**
 * @param {string} s
 * @return {string}
 * 
 * 马拉车算法
 */
var longestPalindrome = function (s) {
  const sWithDollar = Array.from(s).join("$");
  const palindromicRadiuses = [];
  let radius = 0,
    center = 0;
  while (center < sWithDollar.length) {
    while (
      center - (radius + 1) >= 0 &&
      center + (radius + 1) < sWithDollar.length &&
      sWithDollar[center - (radius + 1)] === sWithDollar[center + (radius + 1)]
    ) {
      radius++;
    }
    palindromicRadiuses[center] = radius;

    const leftCenter = center;
    const leftRadius = radius;
    const rightBoundary = leftCenter + leftRadius;
    center++;
    radius = 0;
    while (center <= rightBoundary) {
      const mirrorCenter = leftCenter - (center - leftCenter);
      if (center + palindromicRadiuses[mirrorCenter] < rightBoundary)
        palindromicRadiuses[center] = palindromicRadiuses[mirrorCenter];
      else if (center + palindromicRadiuses[mirrorCenter] > rightBoundary)
        palindromicRadiuses[center] = rightBoundary - center;
      else {
        radius = rightBoundary - center;
        break;
      }
      center++;
    }
  }

  let maxRadiusIndex = 0;
  let maxRadius = 0;
  let maxString = "";
  palindromicRadiuses
    .forEach((value, index) => {
        const targetString = sWithDollar.substring(index-value,index+value+1).replaceAll("$","");
      if (targetString.length >= maxRadius) {
        maxRadius = targetString.length;
        maxString = targetString;
      }
    }); 

   return maxString;
};


/**
 * @param {string} s
 * @return {string}
 * 
 * 暴力但很快
 */
// var longestPalindrome = function (s) {
//   let res = "";
//   let max = 0;

//   for (let i = 0; i < s.length; i++) {
//     for (let j = 0; j <= 1; j++) {
//       let l = i;
//       let r = i + j;

//       while (l >= 0 && r < s.length && s[l] === s[r]) {
//         let len = r - l + 1;
//         if (len > max) {
//           res = s.substring(l, r + 1);
//           max = len;
//         }
//         l--;
//         r++;
//       }
//     }
//   }
//   return res;
// };

console.log(longestPalindrome("abb"));
console.log(longestPalindrome("ccd"));
console.log(longestPalindrome("babad"));
console.log(longestPalindrome("cbbd"));
