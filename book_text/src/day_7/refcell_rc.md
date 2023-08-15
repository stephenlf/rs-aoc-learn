# Shared Mutable References with Rc\<RefCell\>

Rust's answer to the shared mutable reference problem is the `Rc<RefCell<T>>` smart-pointer combo. The `Rc` (or reference counting) pointer lets us bypass the compiler's lifetime checks by ensuring that the data it points to isn't dropped until all references to that data have been dropped. 

This pattern is a component of some languages' [garbage collectors](https://en.wikipedia.org/wiki/Reference_counting), such as [CPython](https://stackify.com/python-garbage-collection/), [Swift](https://www.section.io/engineering-education/automatic-reference-counting-in-swift/) (with ARC), and [PHP](https://www.phpinternalsbook.com/php7/zvals/memory_management.html).

The `Rc` pointer only offers _immutable_ access to its contents. For shared _mutable_ access, we'll need to include the `RefCell` smart pointer as well. `RefCell` moves the burden of checking ownership to _runtime_, rather than compiletime. The result is that we can safely pass around and store multiple mutable references to the same object. When we need to mutate the object behind the reference, we can temporarily crack open the `RefCell` (getting a mutable reference to its contents), and the runtime will make sure we don't already have it open elsewhere. This keeps our code safe.

Together, the `Rc` and `RefCell` smart pointers solve the _lifetime_ and _ownership_ issues, respectively. They also make the language rather verbose, so keep the following pattern in mind:

>> **Creating Rc/RefCell pointers**
>> 
>> We can create smart pointers to objects using the `new` method.
>> ```rust
>> let my_string = String::from("Hello, smart pointers");
>> let my_ref = Rc::new(RefCell::new(my_string));
>> ```
>> Note that `my_ref` now has ownership of the value previously held by `my_string`. `my_string` is no longer a valid variable.
> 
>> **Assigning multiple ownership over `Rc` pointers**
>> 
>> We can assign the `Rc` pointer to multiple variables with the `clone` method. Importantly, assigns clones of the _pointer_, and not the object referenced by the pointer.
>> ```rust
>> let my_string = String::from("Hello, smart pointers");
>> let pointer_1 = Rc::new(RefCell::new(my_string));
>> let pointer_2 = pointer_1.clone();
>> 
>> // alternative syntax
>> let pointer_2 = Rc::clone(&pointer_1);
>> ```
>> Both `pointer_1` and `pointer_2` point to the same memory address. This can be confirmed by querying the memory address of each pointer.
>> ```rust
>> let address_1 = format!("{:p}", pointer_1);
>> let address_2 = format!("{:p}", pointer_2);
>>
>> assert_eq!(address_1, address_2);
>> ```
>
>> **Referencing objects behind `RefCell` pointers**
>> 
>> We can grab an immutable reference to the object referenced by the `RefCell` pointer with the `borrow` method.
>> ```rust
>> let my_string = String::from("Hello, smart pointers");
>> let my_pointer = Rc::new(RefCell::new(my_string));
>>
>> println!("{}", my_pointer.borrow())
>> ```
>
>> **Mutably referencing objects behind `RefCell` pointers**
>> 
>> We can grab a mutable reference to the object behind a `RefCell` pointer with the `borrow_mut` method. We can then modify that object by dereferencing the object with `*`.
>> ```rust
>> let my_pointer = Rc::new(RefCell::new(5));
>>
>> {
>>    let mut reference = my_pointer.borrow_mut();
>>    *reference += 1;
>> }
>>
>> assert_eq!(my_pointer.borrow(), 6);
>> ```
>> Note that `RefCell` will check at _runtime_ whether there is a violation of the borrow checker rules. We still have to make sure that the mutable borrow by `reference` is dropped before the contents are borrowed immutably in the `assert` macro. That is why we put the `reference` variable in a scope that closed before the `assert` statement.
>>
>> In our puzzle, mutable borrows will all occur be created and destroyed within a single method call, so we won't have to worry about manual scoping.

Let's see how these tools can help us model 