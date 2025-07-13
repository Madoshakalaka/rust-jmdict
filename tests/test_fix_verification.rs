#[test]
fn test_language_grouping_fix() {
    use jmdict::{self, Enum};
    
    // Test with default features (English only)
    let entry = jmdict::entries()
        .find(|e| e.number == 1212780)
        .expect("Entry 1212780 should exist");
    
    let sense_count = entry.senses().count();
    let gloss_count: usize = entry.senses()
        .map(|s| s.glosses().count())
        .sum();
    
    println!("With default features (English only):");
    println!("  Senses: {}", sense_count);
    println!("  Total glosses: {}", gloss_count);
    
    // The fix ensures we have 1 sense, not multiple senses for different languages
    assert_eq!(sense_count, 1, "Should have exactly 1 sense");
    assert!(gloss_count > 0, "Should have at least one gloss");
    
    // Verify all glosses are in English
    for sense in entry.senses() {
        for gloss in sense.glosses() {
            assert_eq!(gloss.language.code(), "eng", "All glosses should be in English");
        }
    }
}

#[test] 
#[cfg(all(
    feature = "translations-eng",
    feature = "translations-ger", 
    feature = "translations-fre"
))]
fn test_multiple_languages() {
    use jmdict::{self, Enum};
    use std::collections::HashSet;
    
    let entry = jmdict::entries()
        .find(|e| e.number == 1212780)
        .expect("Entry 1212780 should exist");
    
    let sense_count = entry.senses().count();
    let mut languages = HashSet::new();
    
    for sense in entry.senses() {
        for gloss in sense.glosses() {
            languages.insert(gloss.language.code());
        }
    }
    
    println!("With multiple language features:");
    println!("  Senses: {}", sense_count);
    println!("  Languages: {:?}", languages);
    
    // Should still have 1 sense after the fix
    assert_eq!(sense_count, 1, "Should have exactly 1 sense even with multiple languages");
    
    // Should have glosses in the enabled languages
    assert!(languages.contains("eng"), "Should have English glosses");
    assert!(languages.contains("ger"), "Should have German glosses");
    assert!(languages.contains("fre"), "Should have French glosses");
}