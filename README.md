# Arcan Kernel

> ### A secure, memory-safe operating system kernel written in Rust.

---

**Arcan** (derived from _Marc_ and _Alvan_) is an experimental operating system kernel built from the ground up in Rust.

### Why??

Okay let's all face the hash truth. I didn't build this kernel to be the next big thing or anything like that.
To be honest, I built it because something felt missing, our data is all over the place, and for the average joe,
you can't hide the fact that you have a digital profile all around you.

So yeah I built Arcan to provide an environment where you can be you even for the average user,
removing the complex parts so that you can use your system the way you want to. With no one watching you.
What I want I want :

- **security!!**
- **memory safety!!**
- **a foundation for inter-kernel communication**

### What "Secure" Means to Me

Okay security, I don't think it should mean antivirus software, more bloatware, more surveillance. It should be something baked directly into the silicon of the machine. Though it may be a bit of a departure from traditional security models. Kernel level security is far better than user space security if done well.

And to me this is:

- **Memory Safety by Design**: Eliminating buffer overflows and use-after-free vulnerabilities at the compiler level using Rust.
- **Hardware-Enforced Identity**: A "Burned Hash" root of trust that validates every component before execution.

- **Fine-Grained Compartmentalization**: The **Insider Allocator** ("Inalloc") ensures processes only access memory they are explicitly authorized to use.

- **Custom Secure Protocol**: A proprietary inter-kernel communication layer ("a Verified Datagram Protocol") that replaces insecure standard stacks with identity-based routing.

---
