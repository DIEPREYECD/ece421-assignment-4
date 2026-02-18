## Question 2: Refer to the function cons (https://docs.rs/im/5.0.0/im/list/fn.cons.html) and
a- provide an explanation of the function, the answer should provide a description of what the
function does, and a detailed explanation of each parameter of the function.
### Answer:
The cons function simply prepends a value to a list, adds the value to the front of the list. The first argument it takes is the value, `car` of type `T` to be prepended and the second argument is a list containing elements of type `T`,  `cdr: List<T>`, which must be borrowable , of which the `car` is to be added to the front of then it returns a new `List<T>` with `car` in front of the other elements.

