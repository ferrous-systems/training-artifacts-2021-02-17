Rust has two important traits for thread safety:

* Send - Data can be safely SENT from one thread to another
    * "Transfer of ownership"
* Sync - Data can be safely SHARED from one thread to another
    * "Shared Borrow"

These are "Marker Traits"

## Send

```rust
// NOT safe to send
struct Container1<'a> {
    data: &'a str,
}

// IS safe to send (lives forever)
struct Container2<'static> { // This is "owned" data
    data: &'static str,
}

struct BoringContainer { // ALSO of lifetime 'static
    x: u32
}

fn generic<T: 'static>(T) { // This will work for any data that is OWNED

}

let text = "abcdefg"; // &'static str

{
    let x = String::from("foo"); //

    // Note: `'x` is not real syntax, just a demonstration
    let t: Container1<'x> = Container1 { data: &x }; // OKAY
    let u: Container2<'x> = Container2 { data: &x }; // NOT OKAY

    let a: Container1<'static> = Container1 { data: text }; // OKAY
    let b: Container2<'static> = Container2 { data: text }; // OKAY

    // !!!!
    drop(b);
    drop(a);
    drop(u);
    drop(t); // Does `x` live longer than `t`? --------t
    drop(x); //                                --------x
}
```

If we **own** a Container, why can't we `Send` it to another thread?


fn a
    fn b
    declare our &str called A
        fn c
            spawns a thread called x, passes a reference to A


fn x
    sleep(get_user_input())
    print &str
    returns

How heap allocations work: Split into two parts:

* Data on the Heap
* Handle on the Stack

For example, a String:

* a chunk of bytes on the heap, this contains our actual UTF-8 data
* Some metadata on the stack, including:
    * Pointer to the heap
    * current length (size of the USED part of the heap allocation)
    * current capacity (size of the heap allocation)

What happens when we pass ownership of a string from one function to another?

```rust
fn main() {
    let x = String::from("HUGE STRING...");
    let y = String::from("HUGE STRING...");
    let z = String::from("HUGE STRING...");
    let a = String::from("HUGE STRING...");
    let b = String::from("HUGE STRING...");
    let t = &x;
    let u = &x;
    let v = &x;
    let w = &x;
    let d = &x;
}
```

## Sync

Why would it be bad to share data between two threads?

* Forget to lock/protect some part of shared data
    * Critical Section
* Data corruption
    * "read tearing" or "write tearing"
* Data races

What rules could we make?

* it must be immutible
* It must live "forever"

### Inner mutability

* This means that the "outer" structure is immutible, e.g. we have a `&` shared reference
* BUT, it is possible to somehow get an `&mut` reference to the data

### Thinking about mutexes

Rules:

* No one should read while someone is writing
    * Ownership is part of the story
    * Only the owner can get a mutable reference
    * Hint: It involves ownership + RAII

What if:

1. We (somehow) checked and set a flag that the mutex is "Taken"
    * This flag could be an `AtomicBool`, and we could use CAS - or Compare and Swap operations
    * Operations on `Atomic*` types ONLY EVER REQURE `&` references, even to modify
        * `AtomicU8::fetch_add(5, Ordering::SeqCst) -> u8`
2. If the mutex was take-able, we return a structure, called a "Guard"
3. This guard has a method that lets us borrow an `&mut` "exclusive" reference
    * This `&mut` has a lifetime that is <= the guard itself
4. When the guard is dropped, we release the mutex

## About Send and Sync

* When ALL fields of a structure or enum or tuple are `Send`, then the outer structure is also `Send` automatically.
* When ALL fields of a structure or enum or tuple are `Sync`, then the outer structure is also `Sync` automatically.
* It is possible to "manually" implement `Send` or `Sync` for a data type, BUT, the trait is `unsafe` to implement.

```rust
// Send? - Yes!

struct Container {
    a: u32,
    b: i64,
    c: String,
}
```

```rust
// Sync? -

struct Container {
    a: u32,
    b: i64,
    c: String,
}
```

## We can use some "wrappers" to help us meet Send and Sync (mostly Sync)

### Rule 1: it must be immutible (on the outside, at least)

We can use something like a `Mutex` to move the mutability to "Inner Mutability"

```rust
// Sync!
static MY_DATA: Mutex<String> = Mutex::new(String::from("sneaky"));

fn main() {
    // Returns a result - OK if we have a guard, Err otherwise
    //      vvvvvv - Only requires a `&` shared reference to lock!
    MY_DATA.lock().unwrap();
}
```

### Rule 2: It must live "forever"

* We could make it `static`!
* If we "forget" the handle, and don't call the destructor, no way to drop!
    * Leaking memory is "safe" by Rust's standards. (specifically "Memory Safe")
    * Rust has a way to remove an item WITHOUT calling it's destructor.
        * this is called `std::mem::forget(T)`
* Reference Counter (or other similar techniques)
