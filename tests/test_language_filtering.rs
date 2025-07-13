#[test]
fn test_language_filtering() {
    use jmdict::{self, Enum};
    
    // Find the entry for 換気 (ventilation) 
    let entry = jmdict::entries()
        .find(|e| e.number == 1212780)
        .expect("Entry 1212780 should exist");
    
    println!("Testing entry {} (換気 - ventilation)", entry.number);
    
    let mut gloss_languages = std::collections::HashSet::new();
    let mut gloss_count = 0;
    
    for sense in entry.senses() {
        for gloss in sense.glosses() {
            gloss_count += 1;
            gloss_languages.insert(gloss.language.code());
            println!("  Found gloss: {} (lang: {})", gloss.text, gloss.language.code());
        }
    }
    
    println!("\nTotal glosses: {}", gloss_count);
    println!("Languages found: {:?}", gloss_languages);
    
    // With default features (only English), we should only see English glosses
    assert!(gloss_languages.len() == 1, "Expected only 1 language, found {}", gloss_languages.len());
    assert!(gloss_languages.contains("eng"), "Expected English glosses");
    assert!(gloss_count <= 10, "Expected reasonable number of glosses, found {}", gloss_count);
}