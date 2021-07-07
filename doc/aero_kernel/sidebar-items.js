initSidebarItems({"constant":[["ASCII_INTRO",""],["STACK_SIZE",""]],"fn":[["kernel_ap_startup",""],["kernel_main",""],["kernel_main_thread",""]],"macro":[["module_init",""]],"mod":[["acpi","The ACPI (Advanced Configuration and Power Interface) tables help to gather the CPU, interrupt, and timer informations."],["apic",""],["arch",""],["drivers",""],["fs",""],["logger",""],["mem",""],["modules","A kernel module is an object file that contains code that can extend the kernel functionality at runtime. When a kernel module is no longer needed, it can be unloaded. Most of the device drivers are used in the form of kernel modules."],["prelude",""],["rendy",""],["syscall","System Calls are used to call a kernel service from user land."],["tests",""],["time","The PIT (Programmable Interval Timer) chip basically consists of an oscillator, a prescaler and 3 independent frequency dividers and it is used to create time intervals and calculate estimate time since epoch."],["tls","Thread Local Storage (TLS) are per-thread global variables. On 64-bit each CPU core’s `fs` GDT segment points to the thread local memory area where the thread local static’s live. TLS statics are simply accessed through an offset from `fs`."],["unwind",""],["userland",""],["utils",""]],"static":[["AERO_SYSTEM_ALLOCATOR",""],["CPU_ID",""],["FRAMEBUFFER_TAG","We are now going to define a framebuffer header tag. This tag tells the bootloader that we want a graphical framebuffer instead of a CGA-compatible text mode. Omitting this tag will make the bootloader default to text mode, if available."],["PAGING_TAG","We are now going to define a level 5 paging header tag. This tag tells the bootloader to enable the LEVEL_5_PAGING bit in the Cr4 register. This is not possible to implement in the kernel as we can only enable it in protected mode."],["PHYSICAL_MEMORY_OFFSET",""],["STACK","We need to tell the stivale bootloader where we want our stack to be. We are going to allocate our stack as an uninitialised array in .bss."],["STIVALE_HDR","The stivale2 specification says we need to define a “header structure”. This structure needs to reside in the .stivale2hdr ELF section in order for the bootloader to find it. We use the #[linker_section] and #[used] macros to tell the compiler to put the following structure in said section."]]});