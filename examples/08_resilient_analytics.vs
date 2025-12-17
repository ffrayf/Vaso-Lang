// 08_resilient_analytics.vs
// DEMO: Why Vaso beats Python in Fault Tolerance.
// Python crashes on dirty data. Vaso assimilates it.

print("--- FINANCIAL AUDIT SYSTEM v2.2 ---");

// 1. Simulating a Dataset with CORRUPTED Data
// Python would need a try-catch block for every item.
// Vaso uses native V-Bits.

var reg_norte := 1200;
var reg_sur   := 900;
var reg_este  := error; // ⚠️ DATA CORRUPTION (API Down)
var reg_oeste := 1500;

print("\n[INPUT DATA STREAM]");
print("Region Norte: 1200");
print("Region Sur:   900");
print("Region Este:  CRITICAL_FAILURE");
print("Region Oeste: 1500");

// 2. The Logic: Calculate Total Revenue
// In Python: 1200 + 900 + Error = CRASH (TypeError)
// In Vaso: It propagates the state logically.

var total_revenue := 0;
print("\n--- PROCESSING BATCH ---");

// Processing Norte
total_revenue += reg_norte;
print("Adding Norte... OK. Subtotal:");
print(total_revenue);

// Processing Sur
total_revenue += reg_sur;
print("Adding Sur...   OK. Subtotal:");
print(total_revenue);

// Processing Este (The Killer)
print("Adding Este...  (Detecting Anomaly)");
total_revenue += reg_este; 

// 3. The "Better Than Python" Moment
// Instead of crashing, Vaso absorbs the error state into the variable.
// Now 'total_revenue' carries the information that the calculation is tainted.

print("Current State of Revenue:");
print(total_revenue); // Will print: error("Generic Error")

// 4. Intelligent Recovery (Decision Making)
match total_revenue {
    on => {
        print("✅ Report Generated Successfully.");
    }
    error => {
        print("\n⚠️ ALERT: Tainted Dataset Detected.");
        print("   The calculation contains invalid data points.");
        print("   >>> Switching to PARTIAL RECOVERY mode...");
        
        // We recover the valid data ignoring the error
        // (Simulating a re-calculation excluding the bad node)
        var safe_total := 0;
        safe_total += reg_norte;
        safe_total += reg_sur;
        safe_total += reg_oeste;
        
        print("   ✅ Recovered Valid Revenue:");
        print(safe_total);
        print("   (Region 'Este' was excluded from report)");
    }
}

print("\n--- SYSTEM STATUS: ONLINE (No Crashes) ---");