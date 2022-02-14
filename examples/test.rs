use resources_usage;

fn main() -> std::io::Result<()> {
    let mut r_use = resources_usage::RUsage::new(None);
    r_use.calculate();
    println!("{:?}", r_use.stat);

    Ok(())
}