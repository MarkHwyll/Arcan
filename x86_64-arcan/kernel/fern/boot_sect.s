;org 0x7C00
BITS 16
[org 0x7C00]

;loop:
;  jmp loop
;%include "disk.s"
%include "print.s"
mov ah, 0x0e
mov al, 'H'
int 0x80
hello:
  db "Hello from arcan", 0

mov bl, [hello]
call print_string

jmp $ ;;Jump forever.

times 510 - ($-$$) db 0
dw 0xAA55
