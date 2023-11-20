    let mut md3 = String::new();
    io::stdin().read_line(&mut md3).expect("Invalid String.");
    let chronic_kidney_disease:f32 = md3.trim().parse().expect("Invalid medical diagnosis.");
    
    let mut md4 = String::new();
    io::stdin().read_line(&mut md4).expect("Invalid String.");
    let diabetes:f32 = md4.trim().parse().expect("Invalid medical diagnosis.");

    let mut md5 = String::new();
    io::stdin().read_line(&mut md5).expect("Invalid String.");
    let arthritis:f32 = md5.trim().parse().expect("Invalid medical diagnosis.");

    // input village of residence 
    println!("Enter your village of residence.");

    let mut v1 = String::new();
    io::stdin().read_line(&mut v1).expect("Invalid String.");
    let akpabom:f32 = v1.trim().parse().expect("Invalid village.");

    let mut v2 = String::new();
    io::stdin().read_line(&mut v2).expect("Invalid String.");
    let ngbauji:f32 = v2.trim().parse().expect("Invalid village.");

    let mut v3 = String::new();
    io::stdin().read_line(&mut v3).expect("Invalid String.");
    let atabrikang:f32 = v3.trim().parse().expect("Invalid village.");

    let mut v4 = String::new();
    io::stdin().read_line(&mut v4).expect("Invalid String.");
    let okorobilom:f32 = v4.trim().parse().expect("Invalid village.");