/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @param {number} n
 * @return {ListNode}
 */
var removeNthFromEnd = function (head, n) {
  let currentNode = head;
  let targetNode = head;
  while (currentNode.next != undefined) {
    if (n > 0) {
      n--
      targetNode = head;
    } else {
      targetNode = targetNode.next;
    }
    currentNode = currentNode.next;
  }
  if (targetNode == head && n === 1) {
    head = targetNode.next;
    return head;
  }
  if (targetNode.next) {
    targetNode.next = targetNode.next.next;
    return head;
  }
};

