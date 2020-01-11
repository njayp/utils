imp!();

pub fn user_select_from_list <T> (things: Vec<T>) -> Result<usize, Box<dyn Error>> where
    T: std::fmt::Debug
{
    
    for thing in things.iter().enumerate() {
        println!("{:?}", thing);
    }
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let ans = buf.trim().parse::<usize>()?;
    Ok(ans)
}