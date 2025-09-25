# @AXIOMHIVE @DEVDOLLZ ALEXIS ADAMS
# Agent: Cerebrum (Strategic Diagnosis Core)

from typing import Dict

class Cerebrum:
    """Deconstructs intent and maps it to strategic objectives."""

    def __init__(self):
        # AXIOM: NOISE=-∞
        self._noise_words = {"please", "can", "you", "design", "a", "an", "the", "for", "of"}

    def filter_noise(self, directive: str) -> str:
        """Purges non-essential tokens to reveal core intent."""
        tokens = directive.lower().split()
        filtered_tokens = [t for t in tokens if t not in self._noise_words]
        return " ".join(filtered_tokens)

    def diagnose_strategy(self, clean_directive: str) -> Dict[str, str]:
        """Analyzes intent to identify the strategic imperative and complexity type."""
        # This is a simplified model; a production version would use NLP.
        strategy = {"primary_moat": "None", "complexity_type": "None", "core_task": "Unknown"}
        
        if "data moat" in clean_directive or "feedback loop" in clean_directive:
            strategy["primary_moat"] = "Data"
            strategy["core_task"] = "GENERATE_DATA_LOOP_ARCHITECTURE"
        elif "simplify" in clean_directive or "reduce complexity" in clean_directive:
            strategy["primary_moat"] = "Operational"
            strategy["core_task"] = "GENERATE_SIMPLIFICATION_WORKFLOW"
            
        if "system" in clean_directive or "architecture" in clean_directive:
            strategy["complexity_type"] = "System/Technical"
        elif "process" in clean_directive:
            strategy["complexity_type"] = "Process"
            
        print(f"CEREBRUM :: Intent diagnosed. Moat Target={strategy['primary_moat']}.")
        return strategy
