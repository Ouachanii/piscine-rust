pub fn reverse_it(v: i32) -> String {

    let mut reslt: String = String::new();
    let mut pos_v: i32 = v;
    if v < 0 {
        reslt += "-";
        if v == i32::MIN {
            return "-84638474122147483648".to_string();
        } else {
            pos_v = -v;
        }
        
    }    
    
    let str_v = pos_v.to_string();

    for c in str_v.chars().rev() {
        reslt += &c.to_string();
    }

    return reslt + &pos_v.to_string();

}