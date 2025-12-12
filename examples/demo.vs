```vaso
// Vaso Language Demo
// "Keep it Simple" Philosophy

fn main() {
    print("Welcome to Vaso.");

    // The 'use' block allows us to borrow Python's power instantly
    use(python): {
        def get_ai_data():
            return "Data from Python Libraries"
    }

    // Native Vaso syntax handling the result
    val data := call python.get_ai_data();
    
    // Using Vaso's unique 5-state logic (0-4)
    // 0=No, 1=Yes, 2=Unknown, 3=Paradox, 4=Null
    val is_valid : vbit = 3; 

    match is_valid {
        case 1: print("Data is valid");
        case 3: print("Paradox detected: Data is both true and false");
        else:   print("Unknown state");
    }
}