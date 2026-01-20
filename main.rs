fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> 
{
    let mut num = m;
//     let mut primes: Vec<u64> = Vec::new();
    let mut last_prime = None;
    
    while num <= n
    {   
        if num % 2 != 0
        {
            if is_prime(num)
            {
                if let Some(prev) = last_prime 
                {
                    if num - prev == g as u64 
                    {
                        return Some((prev, num));
                    }
                }
                last_prime = Some(num);
            } 
        }

        num += 1;
    }
    
    
//     let prm_diff: Vec<_> = primes.windows(2).map(|w| w[1]-w[0]).collect();
    
//     if prm_diff.contains(&(g as u64))
//     {
//         let index = prm_diff.iter().position(|&x| x == g as u64);
//         return Some( (primes[index.unwrap()], primes[index.unwrap()+1]));
//     }
//     else
//     {
//       return None;  
//     }
    
   
    
    return None;
}

fn is_prime(number: u64) -> bool
{
    if number < 2 { return false; }
    if number == 2 || number == 3 || number == 5 { return true; }
    if number % 2 == 0 || number % 3 == 0 || number % 5 == 0 || number % 7 == 0 { return false; }
    
    let mut divisor = 5;
    let limit = (number as f64).sqrt() as u64;
    while divisor <= limit
    {
        if number%divisor == 0  || number % (divisor + 2) == 0 { return false; }
        divisor += 6;
    }
    return true;
}
