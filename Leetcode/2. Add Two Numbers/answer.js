/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
var addTwoNumbers = function (l1, l2) {
  let carry = 0;
  let lastNode = null;
  let firstNode = null;
  while (l1 || l2 || carry > 0) {
    const l1Value = l1 ? l1.val : 0;
    l1 = l1 ? l1.next : null;
    const l2Value = l2 ? l2.val : 0;
    l2 = l2 ? l2.next : null;
    let sum = l1Value + l2Value + carry;
    carry = 0;
    if (sum >= 10) {
      sum -= 10;
      carry = 1;
    }
    const thisNode = new ListNode(sum);
    if (lastNode !== null) {
      lastNode.next = thisNode;
    } else {
      firstNode = thisNode;
    }
    lastNode = thisNode;
  }
  return firstNode;
};
