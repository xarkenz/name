    .text
main: 
    # I'm still here!
    add          $t0,$t2,$t3
    sub          $t4, $t5, $t6  # as if
    sll          $s0, $s0, 5
    srl          $s5, $s7, 10
    xor          $t7, $t8, $t9
    lui          $t0, 50
    ori          $t0, $t1, 50
    ori          $t0, $t0, 0x50
    ori          $t0, $t0, 050
    ori          $t0, $t0, 0b1010
#    beq          $t1, $t1, bad      # Bad label
#    lb           $t0, 0x50($t1)
#    lb           $t0, 50($t1)
#    lb           $t0, ($t1)
    beq          $s0, $s0, skip
#    badmnemonic     $t0, $t0, $t0   # Bad instruction
test:
    j           skip
skip:
    jal         done
done:
    add         $t0, $zero, $zero
    j           exit
exit:
    lui         $v0, 10
    srl         $v0, $v0, 16
    syscall