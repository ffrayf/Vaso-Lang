// 03_data_ingestion.vs
// Data processing pipeline using dynamic Arrays and For-Loops.

print("--- LOG INGESTION SERVICE ---");

// Dynamic list of raw log entries
var raw_logs := ["INFO: Service started", "WARN: High latency", "ERROR: DB timeout", "INFO: Health check OK"];

print("Incoming Log Batch:");
print(raw_logs);

var error_count := 0;
var processed_count := 0;

print("\n--- PROCESSING BATCH ---");

for entry in raw_logs {
    processed_count += 1;
    print("Parsing entry:");
    print(entry);

    // Logic: Identify errors in the stream
    // (Simulating string contains check via strict comparison for this demo)
    if entry == "ERROR: DB timeout" {
        print("   >>> ALERT: Error detected in log stream.");
        error_count += 1;
    }
}

print("\n--- BATCH SUMMARY ---");
print("Total Processed:");
print(processed_count);

print("Errors Found:");
print(error_count);

if error_count > 0 {
    print("Status: UNHEALTHY. Alerts triggered.");
} else {
    print("Status: HEALTHY.");
}