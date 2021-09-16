use pyo3::prelude::*;

#[pyfunction]
fn sieve_of_eratosthenes(ceiling: u32) {
    let mut prime: Vec<bool> = Vec::with_capacity((ceiling + 1) as usize);
    for _ in 0..ceiling + 1 {
        prime.push(true)
    }

    let mut p: u32 = 2;
    while p * p <= ceiling {
        if prime[p as usize] == true {
            for i in ((p.pow(2))..ceiling + 1).step_by(p as usize) {
                prime[i as usize] = false
            }
        }
        p += 1
    }
    prime[0] = false;
    prime[1] = false;
    for another_p in 0..ceiling + 1 {
        if prime[another_p as usize] == true {
            println!("{}", another_p.to_string());
        }
    }
}

#[pymodule]
fn sieve_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sieve_of_eratosthenes, m)?)?;

    Ok(())
}
