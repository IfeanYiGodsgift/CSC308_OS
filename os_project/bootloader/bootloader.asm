[BITS 16]
[ORG 0x7C00]

mov ah, 0x0E
mov al, 'O'
int 0x10
mov al, 'S'
int 0x10

cli
hlt

times 510-($-$$) db 0
db 0x55, 0xAA