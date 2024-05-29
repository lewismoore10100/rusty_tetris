pub fn calculate_score(removed_rows: u32) -> u32{
    match removed_rows {
        1 => 40,
        2 => 100,
        3 => 300,
        4 => 1200,
        _ => 0
    }
}