# ApiHasher
ApiHasher is a collection of a few macros designed to perform compile time API hashing. It uses proc macros with lazy_static to lazily evaluate a key at compile time. Using either the struct or const macro depending on your requirements, the macros will create constants as either u64s or structs. At compile time, the randomised key will be used with the provided strings in order to perform DJB2 hashing. These values can then be passed to custom versions of getprocaddress or getmodulehandle, whatever else you want to hide in your malware, in order to disguise your library loads etc.

If you're not using DJB2, there's a one liner in the macros that could be modified to CRC etc to match your needs.

# Usage

Usage is fair simple. Use your preferred macro in order to create consts of your hashes. Here's some example code:

```
use apihasher::{api_hash_consts, api_hash_structs};

api_hash_consts!("NtCreateThreadEx", "NtWriteVirtualMemory");
api_hash_structs!("NtAllocateVirtualMemory", "NtQueueApcThread");


fn main() {
    loop {}
}
```

When expanded, it looks like:

```
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use apihasher::{api_hash_consts, api_hash_structs};
const KEY: u64 = 8466u64;
const NTCREATETHREADEXHASH: u64 = 14993612759381616445u64;
const NTWRITEVIRTUALMEMORYHASH: u64 = 732643099005316127u64;
const NTALLOCATEVIRTUALMEMORYHASH: apihasher::ApiHasher = apihasher::ApiHasher {
    hash: 965858335167436985u64,
    key: 8466u64,
};
const NTQUEUEAPCTHREADHASH: apihasher::ApiHasher = apihasher::ApiHasher {
    hash: 15828872079136630853u64,
    key: 8466u64,
};
fn main() {
    loop {}
}
```

When we expand again:

```
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use apihasher::{api_hash_consts, api_hash_structs};
const KEY: u64 = 9977u64;
const NTCREATETHREADEXHASH: u64 = 8752244963334003492u64;
const NTWRITEVIRTUALMEMORYHASH: u64 = 6100906549099953030u64;
const NTALLOCATEVIRTUALMEMORYHASH: apihasher::ApiHasher = apihasher::ApiHasher {
    hash: 4199941531643019968u64,
    key: 9977u64,
};
const NTQUEUEAPCTHREADHASH: apihasher::ApiHasher = apihasher::ApiHasher {
    hash: 9587504283089017900u64,
    key: 9977u64,
};
fn main() {
    loop {}
}
```

The hashes and key have changed, as they will every time. All the consts created are named as the function capitilised with HASH added to the end.

DJB2 as standard is vulnerable to hashing collisons, even more so when you've randomised the seed, so make sure to test payloads using this method.

The crate is available for no_std but needs alloc so needs a custom allocator.
