use resources_usage;

fn main() -> std::io::Result<()> {

    let mut watcher = resources_usage::Watcher::new(Some(653));
    watcher.look();
    watcher.pretty_print();

    Ok(())
}