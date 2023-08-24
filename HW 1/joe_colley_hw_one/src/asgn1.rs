pub mod asgn1 {
    //use std::unimplemented;

    /**  Q1:
     * TASK:
     *  Implement the function hello_world which prints
     * "Hello, world!" using the println! function.
     * Staff solution length: 1 line of code. */
    pub fn hello_world() {
        println!("Hello, world!")
    }

    /** Q2.A:
     * TASK:
     * Implement the function get_third which accepts an integer array (0-indexed)
     * and should return its third element.
     * Assume that a third element exists.
     * Staff solution length: 1 line of code. */
    pub fn get_third(arg: &[i64]) -> i64 {
        arg[2]
    }

    /** Q2.B
     * TASK:
     * Implement the function powers which accepts an integer x
     * and returns a 3-element integer vector containing
     * x, x^2, x^3 in that order.
     * Staff solution length: 1 line of code. */
    pub fn powers(arg: i64) -> Vec<i64> {
        vec![arg, i64::pow(arg, 2), i64::pow(arg, 3)]
    }

    /** Q2.C:
     * TASK:
     * Implement the function square_array which accepts a
     * (mutable reference to an) integer array.
     * Modify it in-place, updating every array element x to x^2  
     * Staff solution length: 3 lines of code. */
    pub fn square_array(arg: &mut [i64]) {
        for element in arg.iter_mut() {
            *element = i64::pow(*element, 2);
        }
    }

    /** Q3.A:  
     * TASK:
     * Implement the function reverse_array which accepts a
     * (mutable reference to an) integer array containing
     * exactly 10 elements.
     * Use a for loop to reverse the 10-element array in-place.
     * Staff solution length: 5 lines of code. */
    pub fn reverse_array(arg: &mut [i64]) {
        let length = arg.len();
        for i in 0..length / 2 {
            let temp = arg[i];
            arg[i] = arg[length - 1 - i];
            arg[length - 1 - i] = temp
        }
    }

    /** Q3.B:  
     * TASK:
     * Implement the function sum_to_index which accepts
     * an integer n and returns the sum from i = 1 to n of i^2.
     * Return a sum of zero elements when n < 1.
     * Staff solution length: 5 lines of code.  */
    pub fn sum_to_index(n: i64) -> i64 {
        if n < 1 {
            return 0;
        }

        let mut sum = 0;
        for i in 1..=n {
            sum += i64::pow(i, 2);
        }
        sum
    }

    /** Q3.C:  
     * TASK:
     * Implement the function sum_until_zero which accepts
     * (an immutable reference to) an array of integers.
     * Compute the sum of its elements one-by-one until you an encounter a 0,
     * then return the total count so far.
     * If there is no 0, return the sum of the entire array.
     * Staff solution length: 7 lines of code.  
     */
    pub fn sum_until_zero(arg: &[i64]) -> i64 {
        let mut sum = 0;
        for element in arg {
            if *element == 0 {
                return sum;
            }
            sum += *element;
        }
        sum
    }

    /*
       This definition is provided for you to use in a following task.
       You do not need to edit the definition in any way.
       It defines a type named IntMap, which represents a map
       from integers to integers. The type is defined recursively.
       Empty represents an empty map.
       Node(l,k,v,r)  represents a nonempty map where looking up the key  "k"
       would return the value "v". Map l contains all keys less than k.
       Map r contains all keys greater than k. Keys are assumed to be unique.
    */
    #[derive(PartialEq, Eq, Clone)]
    pub enum IntMap {
        Empty,
        Node(Box<IntMap>, i64, i64, Box<IntMap>),
    }

    /** Q4.
     * TASK:
     * Implement the function bsearch, which takes in a (boxed) IntMap and key,
     * then searches the IntMap for the key.
     * Assume that the IntMap is a binary search tree, i.e., it is sorted by key.
     * Do not assume that the given key is in the map; return -1 if not.
     * If it is in the map, return the stored value, which may or may not be -1.
     * A correct solution should take advantage of sortedness and should
     * run in time proportional to tree depth, not total number of nodes.
     * Staff solution length: 11 lines of code.  
     */
    pub fn bsearch(t: Box<IntMap>, key: i64) -> i64 {
        match *t {
            IntMap::Empty => -1,
            IntMap::Node(ref left, k, v, ref right) => {
                //less = bsearch left
                if key < k {
                    bsearch(left.clone(), key)
                //right = bsearch right
                } else if key > k {
                    bsearch(right.clone(), key)
                //equals, return value
                } else {
                    v
                }
            }
        }
    }

    /** Q4.B
     * TODO:
     * Implement the function insert, which takes a boxed map, as well
     * as a key and value, and returns a new map with the key-value pair inserted.
     * Assume the input map is sorted and ensure the output map is sorted.
     * If the key already exists, replace its value with the given value.
     * If not, add the new key with the new value.
     * Staff solution length: 11 lines of code.  
     */
    pub fn insert(t: Box<IntMap>, key: i64, value: i64) -> Box<IntMap> {
        //match = rust's switch statement
        match *t {
            //if empty, make new node with key and value
            IntMap::Empty => Box::new(IntMap::Node(
                Box::new(IntMap::Empty),
                key,
                value,
                Box::new(IntMap::Empty),
            )),
            //if not empty
            IntMap::Node(left, k, v, right) => {
                //less = recursive left insert
                if key < k {
                    Box::new(IntMap::Node(insert(left, key, value), k, v, right))
                //greater = rescursive right insert
                } else if key > k {
                    Box::new(IntMap::Node(left, k, v, insert(right, key, value)))
                    //match on tree, update value
                } else {
                    Box::new(IntMap::Node(left, k, value, right))
                }
            }
        }
    }
}
