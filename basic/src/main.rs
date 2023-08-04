use std::collections::HashMap;

fn main() {
    let mut store: HashMap<String, u32> = HashMap::new();
    store.insert("Java".to_string(), 1);
    store.insert("Rust".to_string(), 2);
    store.insert("TypeScript".to_string(), 3);
    store.insert("C".to_string(), 4);
    println!("{:#?}", store);
    let lung: Option<&u32> = store.get("Java");
    println!("{:#?}", lung);
    store.insert("Java".to_string(), 5);
    println!("{:#?}", store);
    let ret = store.get_key_value(&"Java".to_string());
    println!("{:#?}", ret);
    store.remove("Java");
    println!("{:#?}", store);
    for key in store.keys() {
        println!("{key}");
    }
    for (key, val) in store.iter() {
        println!("key: {key} val: {val}");
    }

    let mut scores = HashMap::new();
    scores.insert("Alice", 10);
    let team_name = "Bob";
    let new_score = 20;

    // エントリーが存在しない場合は新しいエントリーを作成
    let team_score = scores.entry(team_name).or_insert(0);
    // 新しい値を設定
    *team_score += new_score;
    let team_score2 = scores.entry("Alice").or_insert(100);
    *team_score2 += new_score;
    println!("{:#?}", scores);
}
