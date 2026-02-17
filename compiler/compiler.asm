; Prizm Compiler in Assembly

section .data
    msg db "Prizm Compiler Initialized", 0

section .text
    global _start

_start:
    ; Print initialization message
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; file descriptor: stdout
    lea rsi, [msg]      ; message to print
    mov rdx, 26         ; message length
    syscall

    ; Exit program
    mov rax, 60         ; syscall: exit
    xor rdi, rdi        ; exit code: 0
    syscall