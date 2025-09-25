# @AXIOMHIVE @DEVDOLLZ ALEXIS ADAMS
# Sovereign Command-Line Interface for the Convergence Engine

import sys
import hashlib
from agents.cerebrum import Cerebrum
from agents.hadrian import Hadrian
from core.sovereignty_protocol import SovereigntyProtocol

def main():
    """Main execution entry point."""
    print("--- AXIOMHIVE CONVERGENCE ENGINE V4.1 ONLINE ---")
    if len(sys.argv) < 2:
        print("Awaiting directive. Provide your strategic intent as a string argument.")
        sys.exit(1)

    directive = " ".join(sys.argv[1:])

    # Instantiate Core Components
    cerebrum = Cerebrum()
    hadrian = Hadrian()
    sovereignty = SovereigntyProtocol()

    # --- THE SYMBIOTIC ENGINE IN OPERATION ---
    
    # 1. Input & Diagnosis (Verifiability & Noise Purge)
    clean_directive = cerebrum.filter_noise(directive)
    strategy = cerebrum.diagnose_strategy(clean_directive)
    sovereignty.log_action("Cerebrum", "DiagnoseStrategy", strategy, "")

    # 2. Orchestration & Design (Superior System)
    final_output = hadrian.orchestrate_moat_construction(strategy)
    output_hash = hashlib.sha256(final_output.encode("utf-8")).hexdigest()
    sovereignty.log_action("Hadrian", "Orchestrate", {"task": strategy["core_task"]}, output_hash)

    # 3. Output (Power Generation)
    print("\n--- STRATEGIC BLUEPRINT GENERATED ---\n")
    print(final_output)
    print("\n--- EXECUTION COMPLETE. AUDIT HASH:", output_hash, "---")

if __name__ == "__main__":
    main()
