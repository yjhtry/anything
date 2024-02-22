fn main() {
    let id = 222;
    let categories = vec![1, 2, 3];

    let query = categories
        .iter()
        .map(|v| format!("('{}', {})", id, v))
        .collect::<Vec<String>>()
        .join(", ");

    assert_eq!(query, "('222', 1), ('222', 2), ('222', 3)");

    let categories: Vec<i32> = vec![];

    let query = categories
        .iter()
        .map(|v| format!("('{}', {})", id, v))
        .collect::<Vec<String>>()
        .join(", ");

    assert_eq!(query, "");

    let categories = vec![1];

    let query = categories
        .iter()
        .map(|v| format!("('{}', {})", id, v))
        .collect::<Vec<String>>()
        .join("UNION ALL");

    assert_eq!(query, "('222', 1)");

    let nums = vec![1, 2, 3, 4, 5];

    assert_eq!(
        nums.iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(", "),
        "1, 2, 3, 4, 5"
    );
}
