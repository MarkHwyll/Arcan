# Arcan Kernel

> ### A secure, memory-safe operating system kernel written in Rust.

---

**Arcan** (derived from _Marc_ and _Alvan_) is an experimental operating system kernel built from the ground up in Rust.

### Mission

My primary goal is to provide a **secure, memory-safe foundation for inter-kernel communication**.

### What "Secure" Means to Me

When we say "secure," we **do not** mean bolted-on firewalls or antivirus software. We mean the **integration of security primitives directly into the kernel architecture**.

This includes:

- **Memory Safety by Design**: Eliminating buffer overflows and use-after-free vulnerabilities at the compiler level using Rust.
- **Hardware-Enforced Identity**: A "Burned Hash" root of trust that validates every component before execution.

- **Fine-Grained Compartmentalization**: The **Insider Allocator** ("Ialloc") ensures processes only access memory they are explicitly authorized to use.

- **Custom Secure Protocol**: A proprietary inter-kernel communication layer ("a Verified Datagram Protocol") that replaces insecure standard stacks with identity-based routing.

---
