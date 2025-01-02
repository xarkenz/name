# Created by Sean Clarke, January 2025

	# Magic constants for each relevant type of ASCII character
	.eqv		AsciiOther, 0 # Non-alphanumeric
	.eqv		AsciiDigit, 1 # 0-9
	.eqv		AsciiUpper, 2 # A-Z
	.eqv		AsciiLower, 3 # a-z

	.data

# For any character c from 0 to 127, AsciiCharTypeTable[c] is one of the character types above
AsciiCharTypeTable:
	.byte		AsciiOther : 48
	.byte		AsciiDigit : 10 # 0-9
	.byte		AsciiOther : 7
	.byte		AsciiUpper : 26 # A-Z
	.byte		AsciiOther : 6
	.byte		AsciiLower : 26 # a-z
	.byte		AsciiOther : 5

	.text
	# Make the symbol "normalizeString" global so it can be accessed by main.asm
	# Note that "toUpperCase" is not used in any other file, so it does not need to be global
	.globl		normalizeString

# Normalize a string by removing all non-alphanumeric characters and converting all lowercase letters to uppercase.
# This operation is done in-place, so the original string data is modified.
# Input:
# $a0 - The address of the string to normalize. The string must end with a newline character.
# Output:
# $v0 - The length of the normalized string, excluding the newline character.
normalizeString:
	# Push $ra to the stack (another value will be pushed at 4($sp) later)
	addi		$sp, $sp, -8
	sw		$ra, ($sp)
	# $t0 is the address of the current character being read from the string
	move		$t0, $a0
	# $t1 is the address of the next character to overwrite for the normalized string
	# This works because it always holds that $t1 <= $t0
	# ($t0 always increments, but $t1 increments only for alphanumeric characters)
	move		$t1, $a0
normalizeString.char:
	# If the current character being read is a newline, we are done cleaning, so break from the loop
	lbu		$t2, ($t0)
	beq		$t2, '\n', normalizeString.finish
	# If the read character is non-alphanumeric, skip it and move on to the next character
	lb		$t3, AsciiCharTypeTable($t2)
	beq		$t3, AsciiOther, normalizeString.nextChar
	# At this point, the read character is alphanumeric, so append it to the cleaned string
	sb		$t2, ($t1)
	addi		$t1, $t1, 1
normalizeString.nextChar:
	# Continue the cleaning loop with the next character from the original string
	addi		$t0, $t0, 1
	b		normalizeString.char
normalizeString.finish:
	# Ensure the new string ends with a newline ($t2 must be '\n' at this point)
	sb		$t2, ($t1)
	# Calculate the length of the new string as (address of newline) - (address of string)
	sub		$t0, $t1, $a0
	# Push the calculated length to the stack so it doesn't get clobbered by toUppercase
	sw		$t0, 4($sp)
	# Convert all lowercase letters to uppercase ($a0 is already the address of the string)
	jal		toUpperCase
	# Pop the string length into $v0 and restore $ra
	lw		$v0, 4($sp)
	lw		$ra, ($sp)
	addi		$sp, $sp, 8
	# Return to the caller
	jr		$ra

# Convert all lowercase letters in a string to uppercase.
# This operation is done in-place, so the original string is modified.
# Input:
# $a0 - The address of the string to convert. The string must end with a newline character.
toUpperCase:
	# Push $ra to the stack (we technically don't need to do this, but it's good practice)
	addi		$sp, $sp, -4
	sw		$ra, ($sp)
	# $t0 is the address of the current character being read from the string
	move		$t0, $a0
toUpperCase.char:
	# If the current character is a newline, we have reached the end, so break from the loop
	lbu		$t1, ($t0)
	beq		$t1, '\n', toUpperCase.finish
	# If the current character is not a lowercase letter, skip it and move on to the next character
	lb		$t2, AsciiCharTypeTable($t1)
	bne		$t2, AsciiLower, toUpperCase.nextChar
	# At this point, the current character is a lowercase letter, so convert it to uppercase
	# by subtracting 32 from its ASCII value, then store it back in the string
	addi		$t1, $t1, -32
	sb		$t1, ($t0)
toUpperCase.nextChar:
	# Continue the loop with the next character in the string
	addi		$t0, $t0, 1
	b		toUpperCase.char
toUpperCase.finish:
	# Restore $ra by popping from the stack
	lw		$ra, ($sp)
	addi		$sp, $sp, 4
	# Return to the caller
	jr		$ra
