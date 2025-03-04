.section .note.GNU-stack,"",%progbits
.section .text
.global _start

_start:
    // FIXME: Something is causing the core to end up in "a locked up status as a result of an unrecoverable exception"
    //        It's probably a dodgy stack pointer initialisation
    /* Set the stack pointer to the top of the stack */
    ldr sp, =0x20002000

    /* Call your main function */
    bl main

    /* Infinite loop to prevent returning */
.loop:
    b .loop

.global __aeabi_unwind_cpp_pr0
__aeabi_unwind_cpp_pr0:
    bx lr  // Just return, as we are not using C++ exceptions
