/// 階乘
#[inline]
pub fn fac(num: usize) -> usize { 
    let mut total = 1;
    for i in 1..=num { 
        total = total * i;
    }
    total
}

/// 排列算法
/// 从n个无素当中任选k进行排列
#[inline]
pub fn perm(n: usize, k: usize) -> usize { 
    if k > n {  //k不能大于n
        return 0;
    }
    fac(n) / fac(n - k)
}

/// 組合算法
/// 從n個元素當中任选k進行組合
#[inline]
pub fn comb(n: usize, k: usize) -> usize { 
    if k > n { 
        return 0; //k不能大于n
    }
    fac(n) / (fac(k) * fac(n-k))
}
