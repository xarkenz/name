# Created by Sean Clarke, January 2025

	.include	"../SysCalls.asm"
	
	# Maximum length allowed for user input
	.eqv		MaxLength, 200

	.data

PromptMsg:
	.asciiz		"Enter a word or phrase, or nothing to exit: "
PalindromeMsg:
	.asciiz		"Palindrome detected!\n"
NotPalindromeMsg:
	.asciiz		"No palindrome detected.\n"
InputBuffer: # Buffer to hold user input
	.space		MaxLength

	.text
	# Make the symbol "main" global; it is not accessed by any other file, but it must be
	# global in order for the linker to select it as the program entry point
	.globl		main

# Prompt user for a string, then report whether it forms a palindrome.
# Only alphanumeric characters are considered when checking for a palindrome, and case is ignored.
# Repeat this process until no input is entered.
main:
	# Prompt the user for input
	li		$v0, SysPrintString
	la		$a0, PromptMsg
	syscall
	# Read user input into InputBuffer
	li		$v0, SysReadString
	la		$a0, InputBuffer
	li		$a1, MaxLength
	syscall
	# If the first character of InputBuffer is a newline, exit the program
	lbu		$t0, InputBuffer
	beq		$t0, '\n', main.exit
	# Clean up InputBuffer so it only consists of digits and uppercase letters
	la		$a0, InputBuffer
	jal		normalizeString
	# $a0 contains the address of the new string, and $v0 contains the new string length
	# Check for a palindrome from index 0 up to and including (length - 1)
	li		$a1, 0
	addi		$a2, $v0, -1
	jal		isPalindrome
	# If isPalindrome returned true (1), the string was a palindrome
	bnez		$v0, main.palindrome
	# At this point, isPalindrome must have returned false (0)
	# Inform the user that the string was not a palindrome
	li		$v0, SysPrintString
	la		$a0, NotPalindromeMsg
	syscall
	# All finished; return to the input prompt
	b		main
main.palindrome:
	# Inform the user that the string was a palindrome
	li		$v0, SysPrintString
	la		$a0, PalindromeMsg
	syscall
	# All finished; return to the input prompt
	b		main
main.exit:
	# Exit the program
	li		$v0, SysExit
	syscall
