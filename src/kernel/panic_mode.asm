global enter_to_panic_mode
global panic_mode


panic_mode:
    mov ah, 0x0E
    mov al, 'P'
    int 0x10

    mov ah, 0x0E
    mov al, 'A'
    int 0x10

    mov ah, 0x0E
    mov al, 'N'
    int 0x10

    mov ah, 0x0E
    mov al, 'I'
    int 0x10

    mov ah, 0x0E
    mov al, 'C'
    int 0x10  

    ret
