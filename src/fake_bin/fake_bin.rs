pub fn fake_bin(s: &str) -> String {
    s.chars().map(|c| if c < '5' { '0' } else { '1' }).collect()
}

// method chars signature:

// pub fn chars(&self) -> Chars<'_>

// A)
//`(&self)`: The method takes a reference to itself (&self)
// as its only parameter, meaning it does not take ownership
// of the string slice, nor does it mutate it. It operates on
// an immutable reference to a string slice.

//B)
//`-> Chars<'_>`: This indicates the return type of the method.
// Chars is an iterator type defined in the standard library,
// specifically for iterating over characters (char) of a string
// slice. The Chars type is parameterized with a lifetime ('),
// which in this case is elided ('_'). This elision tells
// the compiler to infer the lifetime from the context,
// which effectively means the lifetime of the returned
// Chars iterator is tied to the lifetime of the string slice
// &self it was called on. This ensures that the iterator does not
// outlive the string slice it's iterating over, which would result
// in accessing invalid memory.
