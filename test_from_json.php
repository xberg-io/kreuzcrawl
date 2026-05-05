<?php
declare(strict_types=1);

// Extension is loaded via php-kreuzcrawl.ini or PHP_INI_SCAN_DIR

// Test 1: Simple from_json with minimal JSON
try {
    $config = Kreuzcrawl\CrawlConfig::from_json('{"download_assets":true}');
    echo "✓ Test 1 passed: from_json with minimal JSON\n";
    echo "  Config created: " . get_class($config) . "\n";
} catch (Exception $e) {
    echo "✗ Test 1 failed: " . $e->getMessage() . "\n";
    exit(1);
}

// Test 2: from_json with empty JSON object
try {
    $config2 = Kreuzcrawl\CrawlConfig::from_json('{}');
    echo "✓ Test 2 passed: from_json with empty JSON\n";
} catch (Exception $e) {
    echo "✗ Test 2 failed: " . $e->getMessage() . "\n";
    exit(1);
}

// Test 3: from_json with multiple fields
try {
    $config3 = Kreuzcrawl\CrawlConfig::from_json('{"max_depth":3,"max_pages":100,"respect_robots_txt":true}');
    echo "✓ Test 3 passed: from_json with multiple fields\n";
} catch (Exception $e) {
    echo "✗ Test 3 failed: " . $e->getMessage() . "\n";
    exit(1);
}

echo "\nAll tests passed!\n";
?>
