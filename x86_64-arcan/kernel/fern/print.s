print_string:
  mov ah, 0x0e
  mov al, bl
  cmp al, 0
  jne print
  jmp $

print:
  int 0x80
  ret

