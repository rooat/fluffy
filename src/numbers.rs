/// 計算某個值出現的總次數
pub fn count_all(numbers: &Vec<&str>) -> Vec<usize> { 
    let mut array: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    //array.shrink_to_fit();
    
    for num in numbers { 
        let num = if let Ok(v) = num.parse::<usize>() { v } else { continue; };
        if num >= 10 { 
            continue;
        }
        array[num] += 1;
    }

    array
}

/// 統計某個值出現總的次數
pub fn count_times(numbers: &Vec<&str>) -> Vec<usize> { 
    let mut array: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    //array.shrink_to_fit();
    
    for num in numbers { 
        let num = if let Ok(v) = num.parse::<usize>() { v } else { continue; };
        if num >= 10 { 
            continue;
        }
        array[num] += 1;
    }

    array
}
