## Ownership In Rust
It enables Rust to make memory safety guarantees without needing a garbage collector

First the problem : Every Program needs to manage memory , when a program runs it uses RAM(Memory), to store things like variables 

let x = 10;
Rust must store 10 somewhere in memory.
But eventually that memory must be freed, otherwise programs would keep using memory forever.
This is called memory management.

### Three ways language manages memory !

1. GC (Garbage Collection) : a garbage collector runs and automatically remove unused memory (basically frees it)
2. Manual Memory Management : C/C++ , The programmer must explicitly allocate and free memory.
3. Rust Approach : Ownership
### What is Ownership ?
Ans : Every Piece of memory has a owner. Rust enforces rules about ownership at compile time.

{
    let s = String::from("hello");
}

when s goes out of scope , memory is automatically freed 

Rust 3 Ownership Rules : 

Rule 1 -> Each Value has 1 owner    
Rule 2 -> There can only be 1 owner at a time
Rule 3 -> When the owner get out of scoped, the value is dropped

### How Garbage Collector Works !!!

The problem GC Solves : Your program constantly creates data in memory. The question is — when is it safe to free that memory?

Step-1 The heap : Every object your program creates goes into the heap , the big chunk of RAM 
HEAP MEMORY
┌─────────────────────────────────────┐
│  [User obj]  [List]  [String]  ...  │
│  0x001       0x045   0x089          │
└─────────────────────────────────────┘

The GC's job is to look at this heap and figure out which objects are still reachable by your program and which are just taking up space.

The thing here is Reachability : can your program still reach this object through any chain of reference ?
If yes then keep it , if no free it !



──────────────────────────────────────────────
Program runs → GC PAUSE → Program runs → GC PAUSE → ...
               (10-100ms)               (10-100ms)
This is what people mean by GC pauses. In Python and older Java this was a real problem — your server would randomly freeze for 100ms every few seconds. For a web server handling thousands of requests, that's a disaster.


### What Rust does Instead !!!
No runtime tracking. No pauses. No reference count overhead. No cycle detection. The binary that gets produced already has the free instructions in exactly the right places — figured out once at compile time, never thought about again.

Well Rust uses drop() and free() : both at different layers !!!

Layer-1 : drop()->Rust Level
When a value goes out of scope, Rust calls drop() on it. This is a Rust trait — Drop. It runs any cleanup logic first.

Layer-2 : dealloc() (Rust Allocater Level)
After drop() runs the clean-up, rust calls dealloc() from its allocator API : this is actually memory release step

Layer-3 : free() (C/OS Level)
`dealloc()` by default calls the system allocator — which on Linux/Mac is `malloc`/`free` from libc. So ultimately yes, `free()` does get called at the lowest level.
Rust drop() → Rust dealloc() → libc free() → OS gets memory back

### The key difference from Manual Memory Management of C
In C — you call free() and you can get it wrong.
In Rust — the compiler inserts the drop() call in the right place, which chains down to free(). You never call either manually.


### Move vs Copy 
Rust has two completely different behaviour depending on the types

Heap Types - they move 
let s1 = String::from("hello");
let s2 = s1;   // s1 is MOVED into s2

println!("{s1}"); // ❌ COMPILE ERROR — s1 no longer exists
println!("{s2}"); // ✅ fine


Stack types - they copy 
let x = 5;
let y = x;   // x is COPIED into y

println!("{x}"); // ✅ fine — x still valid
println!("{y}"); // ✅ fine

The rule: If a type implements the Copy trait, it copies. If it doesn't, it moves.

* Types that are Copy (safe to copy, stack-only):

All integers: i32, u64, etc.
Floats: f64, f32
bool
char
Tuples — only if everything inside is also Copy

* Types that are NOT Copy (heap-allocated, they move):

String
Vec<T>
Box<T>
Anything you define with heap data inside

### What Happens In Memory During a Function Call
Every function gets its own stack frame , a dedicated chunk of stack memory that exists only while that function is running.

STACK
─────
main()        ← running
  ...

dangle()      ← called, gets its own frame
  s
   ├─ ptr  ──────────────────► ["hello"]  (on heap)
   ├─ len: 5
   └─ cap: 5

When dangle finishes , its entire stack frame is wiped. Every variable inside it ceases to exist

* What dangling means 
-> Step1 : dangle() starts, s is created
STACK                         HEAP
─────                         ────
dangle():
  s
   ├─ ptr        ───────────► ["hello"]
   ├─ len: 5
   └─ cap: 5

-> Step2 : &s is created to return 

-> step3 : Funtion returns , stack frame is WIPED
STACK                         HEAP
─────                         ────
dangle():
  s  ❌ GONE                 ["hello"]  ← freed, memory is gone
   ├─ ptr  ❌                    ✖
   ├─ len  ❌
   └─ cap  ❌

  &s  ──────────► s ❌ ──────────✖
        points to               memory
        nothing                 freed
    
Step4 -? Caller Receives the reference
STACK                         HEAP
─────                         ────
main():
  result = &s  ──────────────► ???
                (points to memory that was freed)
                (could be anything now — another variable,
                 garbage data, zeros, anything)

* Code for the reference
fn dangle() -> &String {    // ❌ COMPILE ERROR
    let s = String::from("hello");
    &s   // return a reference to s
}   // s is dropped here — reference now points to freed memory

This is dangling pointer.Result holds an address that used to have "Hello", but the memory has been freed and now there it could be anything 

This things crashes out C program 

* But what Rust does : Lifetime check at the compile time
Rust tracks how long every value lives. That's called a lifetime.

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

How compiler think through this !
s is created inside dangle() -> s's lifetime starts here
&s is a reference to s -> reference can only be valid as long as s is alive 

function return &s to the caller
  → but s's lifetime ENDS when the function ends
  → reference would outlive the thing it points to
  → IMPOSSIBLE — refuse to compile

The fix is simple : Instead of reference , return the ownership

What happens in the memory !

```
STACK                         HEAP
─────                         ────
no_dangle():
  s
   ├─ ptr        ───────────► ["hello"]
   ├─ len: 5
   └─ cap: 5
         │
         │  OWNERSHIP MOVES OUT
         ▼
main():
  result
   ├─ ptr        ───────────► ["hello"]  (still alive, ownership transferred)
   ├─ len: 5
   └─ cap: 5
```

* Golden Rule 

A reference can never outlive the value it points to.

### Data Races

First what is race condition ?
A race condition is when the output of your program depends on the timing of events — and timing is unpredictable.

Two people editing the same Google Doc simultaneously:

Person A reads:  "Hello world"
Person B reads:  "Hello world"

Person A writes: "Hello Rust"    (saves)
Person B writes: "Hello Python"  (saves)

Final result: "Hello Python"
Person A's change is GONE — silently lost

This is nightmare for the concurrent programming

Data Race : Specific Dangerous Version
This is the race condition which is specifically in the memory.
Threee conditions must all be true simaltaneously

Condition 1: Two or more pointers access the same memory
Condition 2: At least one of them is WRITING
Condition 3: No synchronization (no locks, no coordination)

All three together = undefined behaviour

HEAP
────
["hello"]  ← both r1 and r2 point here
   ▲  ▲
   │  │
   │  r2 (Thread B — wants to write "hello python")
   │
   r1 (Thread A — wants to write "hello rust")

Now both write at the same time. A string internally has 3 parts ptr,len,cap.
Both the threads are updating all 3 fields.

How Rust Eliminates this ?

-> The one-mutable-borrow rule makes the scenario above physically impossible:
let mut s = String::from("hello");

let r1 = &mut s;   // Thread A gets mutable borrow
let r2 = &mut s;   // ❌ COMPILE ERROR — Thread B can't get one

// The race condition cannot happen
// because r2 cannot exist while r1 exists
```
```
WHAT RUST ALLOWS:           WHAT RUST PREVENTS:
─────────────────           ────────────────────
One writer:                 Two writers:
  r1 ──► s (writing)          r1 ──► s (writing)
                               r2 ──► s (writing) ← BLOCKED

Many readers:               Reader + Writer:
  r1 ──► s (reading)          r1 ──► s (reading)
  r2 ──► s (reading)          r2 ──► s (writing) ← BLOCKED
  r3 ──► s (reading)

### NLL (Non Lexical Lifetimes)

Lexical means "based on the return structure of code — specifically, the `{` and `}` braces.

The old (wrong) mental model most people start with:

```
"A borrow lasts from where it's created to the closing } of its scope"
```

That's lexical lifetime. And it's **not how Rust actually works.**

The real role - NLL

A borrow lasts from where it's created to its LAST USE.
Not to the closing brace. To the last line that uses it.

This is Non-Lexical Lifetimes — the compiler tracks actual usage, not just scope boundaries.


Lexical vs NLL thinking

Example : You have one special toy. Three kids want to play with it.
The rule is simple:

Only one kid can hold the toy at a time.

Now the old Rule : (Lexical - The dumb version)
Once you pick up the toy, you hold it until the bell rings. Even if you put it down. Even if you walk away. It's still yours until the bell

Bell rings at 3pm.

Alex picks up toy at 1pm
Alex puts toy down at 1:30pm
Alex goes outside to play

Sam wants the toy at 2pm
Teacher says: "NO. Alex still has it until 3pm bell"

Sam: "But Alex put it down an hour ago and left??"
Teacher: "Doesn't matter. Bell hasn't rung."

That's insane. The toy is just sitting there. Nobody is using it. But Sam can't have it.
This is how old Rust worked. Borrow lasts until the closing } brace — the "bell" — even if you stopped using it way earlier.


The new Rule : NLL
You hold the toy until the last moment you actually touch it. The second you put it down and walk away — someone else can pick it up."

Now in rust the same exact idea
let mut s = String::from("hello");

let r1 = &s;                  // Alex picks up toy
let r2 = &s;                  // Sam also looking at toy (fine, reading only)
println!("{r1} and {r2}");    // ← Alex and Sam PUT THE TOY DOWN HERE
                              //   their turn is OVER at this line

let r3 = &mut s;              // ✅ toy is free — someone else can grab it
println!("{r3}");
```
```
TIMELINE:

1pm ──── Alex has toy ────── 1:30pm (puts down) ············ 3pm bell
1pm ──── Sam has toy ─────── 1:30pm (puts down) ············ 3pm bell
                                                  2pm ── New kid has toy ── 3pm bell


When this fails : when the toy is still being used

let mut s = String::from("hello");

let r1 = &s;                  // Alex picks up toy
let r2 = &s;                  // Sam picks up toy
let r3 = &mut s;              // ❌ New kid tries to grab it
                              // BUT Alex and Sam haven't put it down yet!

println!("{r1}, {r2}, {r3}"); // Alex and Sam still using it here
```
```
TIMELINE:

Alex has toy ─────────────────────────────────── still using at 3pm
Sam has toy  ─────────────────────────────────── still using at 3pm
                    New kid tries to grab ────────────────────────────
                         ↑
                    CONFLICT — toy is still being used!
```

New kid can't grab the toy while Alex and Sam are still holding it. Compiler says no.

---

## The One Sentence Summary
```
Your borrow ends the moment you stop using it.
Not when the function ends.
Not when the } closes.
The moment you last touch it — it's free for someone else.

### String vs &str 

let s = String::from("hello");
```
```
STACK                           HEAP
─────                           ────
s
 ├─ ptr      ─────────────────► ["h","e","l","l","o"]
 ├─ len: 5                       (actual characters live here)
 └─ cap: 5
```

Three things happen when you create a `String`:
1. Rust goes to the heap and says "give me some memory"
2. Writes `"hello"` into that memory
3. Puts a pointer + len + cap on the stack so you can find it

`s` on the stack is the **owner** of that heap memory. This is the critical part:
```
s owns the heap memory
  → when s dies, the heap memory is freed
  → when s moves, ownership of heap memory transfers
  → nobody else can own that same heap memory at the same time


&str -> borrows data , own nothing 

let s = "hello";   // type is &str
```
```
BINARY / STACK                  HEAP
──────────────                  ────
"hello" baked                   (nothing here)
into the program
binary itself
    ▲
    │
s ──┘   (just a pointer + length, no heap involved)


### Tuple Struct vs Regular Struct
```
Regular struct:                 Tuple struct:
───────────────                 ─────────────
struct Point {                  struct Point(i32, i32);
    x: i32,
    y: i32,
}

access: point.x, point.y       access: point.0, point.1
clear field names               no field names
use when fields have meaning    use when position is obvious

we use tuple struct over regular struct when we want type safety through naming.
```


### Ownership in methods 

struct Message {
  content : String,
}

impl Message {
    fn show(&self) {
        println!("{}", self.content);  // borrows — content still owned by Message
    }

    fn into_content(self) -> String {
        self.content  // moves content OUT — Message is consumed
    }
}

fn main () {
  let msg = Message { content: String::from("hello") };

  msg.show();                        // borrows — msg still alive
  let content = msg.into_content();  // msg consumed — content moved out
  // msg is gone here
  println!("{}", content);           //  content is alive
}

```
struct    →  defines the data shape
impl      →  defines the behavior

struct User { ... }     data: what a User IS
impl User { ... }       behavior: what a User can DO

Together they form a complete type —
data + behavior in one place
```



### #[derive(Debug)]

This is used to print the struct and for pretty print we use "{:#?}"


### Enum

A struct lets you group related data together. An enum lets you say a value can be one of several possible things.

Each variant can hold different data
enum Message {
    Quit,                        // no data
    Move { x: i32, y: i32 },    // named fields like a struct
    Write(String),               // single String
    ChangeColor(i32, i32, i32),  // three values
}



### Enum vs Structs

Struct — has ALL fields at once:
  User has name AND age AND email — all three always exist

Enum — is ONE of the variants:
  Direction is North OR South OR East OR West — only one at a time

### if let / match

 The Rule — When To Use Which
```
Have 2+ variants to handle differently  →  use match
Only care about 1 variant               →  use if let
Only care about 1 variant + a default   →  use if let + else
```



### Packages Crates Modules Paths 

Packages  →  what Cargo creates when you run cargo new
Crates    →  the actual compiled unit (binary or library)
Modules   →  organize code inside a file or across files
Paths     →  how you refer to things (like std::io::stdin)

Package
  └── Crate(s)
        └── Module(s)
              └── Functions, Structs, Enums, etc.