// 07_transaction_ledger.vs
// ACID Transaction Simulation with Auto-Rollback capability.
// Demonstrates: Data Integrity, State Management, and Logical Branching.

print("--- BANKING CORE: LEDGER SERVICE v2.2 ---");

// 1. Initial Account States
var acc_origin_balance := 5000;
var acc_dest_balance   := 0;
var transaction_id     := 1001;

print("Initial State:");
print("Origin Balance: 5000 | Destination Balance: 0");

// --- TRANSACTION 1: SUCCESSFUL TRANSFER ---
print("\n[TX: 1001] Initiating Transfer of $1500...");

// Step A: Debit Origin
acc_origin_balance -= 1500;
var debit_status := on; // Success

// Step B: Credit Destination
acc_dest_balance += 1500;
var credit_status := on; // Success

// Step C: Verify Integrity (Atomic Check)
var tx_integrity := debit_status;
tx_integrity += credit_status;

match tx_integrity {
    on => {
        print("   >> Debit: OK");
        print("   >> Credit: OK");
        print("   [SUCCESS] Transaction committed to ledger.");
    }
    error => {
        print("   [CRITICAL] Transaction failed.");
    }
}

print("Balances: Origin: 3500 | Dest: 1500");


// --- TRANSACTION 2: FAILED TRANSFER (SIMULATED NETWORK ERROR) ---
print("\n[TX: 1002] Initiating Transfer of $5000...");

// Capture state before modification (Snapshot for Rollback)
var snapshot_origin := acc_origin_balance;
var snapshot_dest   := acc_dest_balance;

// Step A: Debit Origin (Logic check: Sufficient funds?)
// We have 3500, trying to send 5000.
// (Simulating the debit happening anyway to show rollback need)
acc_origin_balance -= 5000; 
var step_1_status := on;

// Step B: Credit Destination (SIMULATING FAILURE)
// Imagine a network timeout or API crash here.
print("   >> Debited $5000 from Origin...");
print("   >> Attempting Credit to Destination...");

// ... Network glitch ...
var step_2_status := error; // "Connection Reset"
print("   >> [ERROR] Connection lost with Destination Bank.");

// Step C: Integrity Check
var tx_final_state := step_1_status;
tx_final_state += step_2_status; // on + error = error

// --- ROLLBACK ENGINE ---
print("\n--- INTEGRITY MONITOR ---");
print("Transaction State:");
print(tx_final_state);

match tx_final_state {
    on => {
        print("Transaction Complete.");
    }
    error => {
        print("🛑 INTEGRITY VIOLATION DETECTED.");
        print("   Data is inconsistent (Money left origin but never arrived).");
        print("   >>> INITIATING AUTOMATIC ROLLBACK sequence...");
        
        // RESTORE DATA FROM SNAPSHOT
        acc_origin_balance = snapshot_origin;
        acc_dest_balance = snapshot_dest;
        
        print("   ✅ Rollback successful. Balances restored.");
    }
}

print("\n--- FINAL AUDIT ---");
print("Origin Balance (Expected 3500):");
print(acc_origin_balance);
print("Dest Balance (Expected 1500):");
print(acc_dest_balance);

print("System Integrity: 100%");