pub fn format_mem(mem: i64) -> String {
    let mem = mem as f64;
    if mem < 1024.0 {
        return format!("{:.3}B", mem);
    }

    let mem = mem / 1024.0;
    if mem < 1024.0 {
        return format!("{:.3}KB", mem);
    }
    let mem = mem / 1024.0;

    if mem < 1024.0 {
        return format!("{:.3}MB", mem);
    }

    let mem = mem / 1024.0;

    return format!("{:.3}GB", mem);
}
