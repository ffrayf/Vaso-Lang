// 02_retry_logic.vs
// Demonstration of control flow (while loops) mimicking a database connection retry.

print("--- DATABASE CONNECTION MODULE ---");

var max_retries := 5;
var attempt := 0;
var connected := 0; // 0: False, 1: True

// Loop until connected or retries exhausted
while attempt < max_retries {
    attempt += 1;
    print("Attempting connection to DB_PRIMARY...");
    
    // Simulate latency
    var _ := Time.sleep(500);

    // Simulation: Connection succeeds on attempt 3
    if attempt == 3 {
        print(">> Handshake received. Connection established.");
        connected := 1;
        // Break mechanism: force loop exit condition
        attempt := 10; 
    } else {
        print(">> Connection timed out. Retrying...");
    }
}

if connected == 1 {
    print("SUCCESS: Database is ready for queries.");
} else {
    print("CRITICAL: Could not reach database after max retries.");
}