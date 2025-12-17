// 04_service_health.vs
// Microservice Health Aggregator using Vaso's State Dominance.
// Demonstrates how error states propagate without try/catch blocks.

print("--- MICROSERVICE HEALTH AGGREGATOR ---");

// 1. Initialize Service States (Default to loading)
var auth_service := loading;
var db_service   := loading;
var api_gateway  := loading;

print("Initializing cluster checks...");

// 2. Simulate Service Status Responses
// Auth is OK
auth_service = on;
print("[Auth Service]: ONLINE");

// DB is OK
db_service = on;
print("[Database]: ONLINE");

// API Gateway fails (e.g., timeout)
// 'unknown' represents a non-critical warning or timeout
api_gateway = unknown; 
print("[API Gateway]: TIMEOUT (Unknown State)");

// 3. Aggregate Health Logic
// In Vaso, states sum up based on hierarchy: Error > Unknown > Loading > On
var cluster_status := auth_service;
cluster_status += db_service;
cluster_status += api_gateway;

print("\n--- CLUSTER STATUS REPORT ---");
print("Aggregate State:");
print(cluster_status); 

// 4. Decision Making (Pattern Matching)
match cluster_status {
    on => {
        print("Status: GREEN. All systems operational.");
    }
    loading => {
        print("Status: YELLOW. Services are warming up.");
    }
    unknown => {
        print("Status: ORANGE. Partial degradation detected. Traffic rerouted.");
    }
    error => {
        print("Status: RED. Critical failure. PagerDuty triggered.");
    }
}