# Created by Sean Clarke, January 2025

	.text
	# Make the symbol "isPalindrome" global so it can be accessed by main.asm
	.globl		isPalindrome

# Determine whether a string forms a palindrome; that is,
# whether the string reads the same both forward and backward.
# Input:
# $a0 - The address of the string to check.
# $a1 - The index of the first character in the string to check.
# $a2 - The index of the last character in the string to check.
# Output:
# $v0 - true (1) if the string forms a palindrome, or false (0) otherwise.
isPalindrome:
	# Push $ra to the stack
	addi		$sp, $sp, -4
	sw		$ra, ($sp)
	# If the range includes 1 or fewer characters, the string is a palindrome, so
	# skip the recursive call and return true (base case)
	sge		$v0, $a1, $a2
	bnez		$v0, isPalindrome.finish
	# Retrieve the character at the lower index
	add		$t0, $a0, $a1
	lbu		$t0, ($t0)
	# Retrieve the character at the upper index
	add		$t1, $a0, $a2
	lbu		$t1, ($t1)
	# If the two characters are not equal, the string is not a palindrome, so
	# skip the recursive call and return false (base case)
	seq		$v0, $t0, $t1
	beqz		$v0, isPalindrome.finish
	# Perform a recursive call of isPalindrome with the check range narrowed by 1 character in both directions
	# This will check all characters between the two that were just checked
	addi		$a1, $a1, 1
	addi		$a2, $a2, -1
	jal		isPalindrome
	# At this point, $v0 contains the value returned by the recursive call; leave it unchanged
isPalindrome.finish:
	# Restore $ra by popping from the stack
	lw		$ra, ($sp)
	addi		$sp, $sp, 4
	# Return to the caller
	jr		$ra
