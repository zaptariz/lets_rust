pub fn data_types() {
    /*---------------------------*|
     *  scalar      *    compound *
     *--------------*-------------*
     *  String      *   Tuple     *
     *  Integer     *   Array     *
     *  Char        *   Vector    *
     *  Bool        *             *
     *----------------------------*| */
//====================================================================================================================================
            // SCALAR
//====================================================================================================================================
    // every integers as 32byte default by rust, without mut keyword we cant re-assign the value of the variable in the scope
    let mut var = 4;
    println!("{} \n{:p}\n ", var, &var);
    var = 7;
    println!("{} \n{:p}\n ", var, &var);

    // can change by shadow variable
    let var: i8 = 30;
    println!(
        "value of shadow   : {} \naddress: {:p}",
        var, &var
    );
    // declaration of charecter
    let charecter = 'A';
    println!("\n{}", charecter);

    // declare a string with in the double quotes
    let z = "lets get rusty\n";
    println!("{}", z);

    // we can declare a empty string , but should use
    // let mut z = String::new();
    // io::stdin().read_line(&mut z).expect("give a string");
    // println!("{}", z);

    
//====================================================================================================================================
            // COMPOUND
//====================================================================================================================================
   
    //declare a tuple variable and destructering    
    let tuple = ("zapt", 23);
    let (name, age) = tuple;
    println!("by destructering \n");
    println!("name : {}\nage : {}", name, age);

    //destruct a tuple by . operator
    println!("by . operator ");
    println!("name : {}\nage : {}", tuple.0, tuple.1);

    // for loop like for each in c, c++ languages
    let array = [1, 2, 3, 4, 5, 6, 7];
    for print_value in array.iter() {
        println!("value : {}", print_value);
    }
    // for loop by given range
    for print_value in 1..5 {
        println!(" value : {}", print_value);
    }

    // use while, loop by length of the array
    let mut len = array.len();
    while len != 0 {
        println!("array value in {} position : {}", len, array[len - 1]);
        len -= 1;
    }
    //const variale could not declared as mutable variable, its always immutable
    //must specify with a data type
    const VARIABLE: i32 = 54;
    println!("value of the const varible : {}", VARIABLE);
}

