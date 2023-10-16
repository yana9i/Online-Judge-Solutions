/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function (nums, target) {
  const map = new Map();
  for (let index = 0; index < nums.length; index++) {
    const substractedTarget = target - nums[index];
    const lastIndex = map.get(nums[index]);
    if (lastIndex !== undefined ) 
        return [lastIndex,index];
    map.set(substractedTarget, index);
  }
};

console.log(twoSum([2, 7, 11, 15], 9));
console.log(twoSum([3, 2, 4], 6));
console.log(twoSum([3, 3], 6));
console.log(twoSum([0, 4, 3, 0], 0));
console.log(twoSum([-1, -2, -3, -4, -5], -8));
