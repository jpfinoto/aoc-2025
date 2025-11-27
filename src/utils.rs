use itertools::Itertools;

pub fn get_cpu_name() -> String {
    let s = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::everything());
    s.cpus()
        .iter()
        .map(|cpu| cpu.brand().trim())
        .dedup_with_count()
        .sorted_by_key(|(count, _)| *count)
        .map(|(count, name)| format!("{}x {}", count, name))
        .join(", ")
        .to_string()
}
