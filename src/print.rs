pub fn run(){ 
    println!("Hello from the print.rs File");           
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);            
    
    let emoji = '\u{1F600}';       
    println!("{:?}", emoji); 

    // String
    let mut res = String::from("Suman");    
    res.push_str("World!");   
    println!("{}", res);   

    println!("Capacity: {}", res.capacity()); 

    println!("{}", res.is_empty());  

    println!("{}",res.contains("Suman"));  
    
    println!("{}",res.replace("Sumam", "World")); 

    for ans in res.split_whitespace(){
        println!("{}",ans); 
    }

    // Create String with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}",s); 

    assert_eq!(2,s.len()); 
    assert_eq!(10,s.capacity());    

    // Tuples
    let person: (&str, &str, i32) = ("Suman", "Chandigarh", 21);
    println!("My name is {}, and i am from {}. My age is {}", person.0,person.1, person.2); 

    // Arrays
    let num: [i32; 5] = [1,2,3,4,5]; 
    println!("{:?}", num[4]);   
    println!("{}", std::mem::size_of_val(&num));  

    let slice: &[i32] = &num[1..3];  
    println!("Slice: {:?}", slice); 

    // vector - Resizable Array 
    let mut number: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10]; 
    number.push(11);
    number.push(20);  
    number.pop();  
    println!("{:?}", number); 

    //Loop through vector values
    for x in number.iter(){
        println!("Number : {:?}", x); 
    }  

    for x in number.iter_mut(){
        *x*=2;       
    }
    println!("{:?}", number); 

    // Pointers and References
      // Primitive  
    let array1 = [1,2,3,4,5];
    let array2 = array1;

    println!("{:?}", (array1, array2)); 
    
    // Vector - Non Primitive
    let vec1 = vec![1,2,3,4,5];
    let vec2 = &vec1;

    println!("{:?}", (&vec1, vec2));    
    
    
    
} 