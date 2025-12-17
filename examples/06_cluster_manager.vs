// 06_cluster_manager.vs
// Global Infrastructure Orchestrator.
// Demonstrates Functions, Stack Memory, Loops, and Fault Tolerance.

print("--- GLOBAL INFRASTRUCTURE MANAGER v2.2 ---");
var timestamp := Time.now();
print("Execution Timestamp:");
print(timestamp);

// --- CONFIGURATION ---
// Simulated node status list: 1=Healthy, 0=Down, 2=High Latency
var region_us_east := [1, 1, 0, 1]; // One node down
var region_eu_west := [1, 1, 1, 1]; // All healthy

// --- LOGIC ---

fn diagnose_node(status_code) {
    if status_code == 1 {
        print("   [INFO] Node Status: HEALTHY.");
    }
    
    if status_code == 2 {
        print("   [WARN] Node Status: HIGH LATENCY. Flagged for review.");
    }

    if status_code == 0 {
        print("   [CRITICAL] Node Status: OFFLINE. Initiating auto-recovery sequence...");
        
        // Auto-recovery loop
        var attempt := 0;
        var recovered := 0;
        
        while attempt < 3 {
            attempt += 1;
            print("      > Sending heartbeat retry...");
            // Simulate recovery success on attempt 2
            if attempt == 2 {
                recovered := 1;
            }
        }
        
        if recovered == 1 {
            print("      [SUCCESS] Node recovered and re-joined cluster.");
        } else {
            print("      [FAILURE] Node unresponsive. Removing from load balancer.");
        }
    }
}

fn scan_region(region_name, nodes_list) {
    print("\n--- SCANNING REGION: ---");
    print(region_name);
    
    var node_id := 0;
    var healthy_count := 0;

    for node_status in nodes_list {
        node_id += 1;
        print(" > Inspecting Node ID:");
        print(node_id);
        
        // Nested function call
        diagnose_node(node_status);
        
        if node_status == 1 {
            healthy_count += 1;
        }
    }

    print("Region Summary (Healthy Nodes):");
    print(healthy_count);
}

// --- EXECUTION ---

print("\n[ORCHESTRATOR] Starting Sequential Scan...");

scan_region("us-east-1 (N. Virginia)", region_us_east);

// Simulated network delay
var _ := Time.sleep(1000);

scan_region("eu-west-1 (Ireland)", region_eu_west);


// --- COST CALCULATION ---
print("\n[FINANCE] Calculating Hourly Burn Rate...");

var cost_per_node := 15; // USD per hour
var total_nodes := 8;
var total_burn := 0;
var i := 0;

while i < total_nodes {
    total_burn += cost_per_node;
    i += 1;
}

print("Estimated Hourly Cost (USD):");
print(total_burn);

print("\n--- ORCHESTRATION COMPLETE ---");